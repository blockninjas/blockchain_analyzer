use super::ClusterUnifier;
use bir;
use config::Config;
use db_persistence::{self, *};
use diesel::{self, prelude::*};
use failure::Error;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::result::Result;
use std::sync::Mutex;
use task_manager::{Index, Task};

pub struct ClusteringTask {}

impl ClusteringTask {
    pub fn new() -> ClusteringTask {
        ClusteringTask {}
    }
}

impl Task for ClusteringTask {
    fn run(
        &self,
        config: &Config,
        db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<(), Error> {
        info!("Run ClusteringTask");

        let transactions = bir::read_bir_files(&config.resolved_bir_file_path)?
            .into_iter()
            .map(
                |path| File::open(path).unwrap(), // TODO Return error instead of panicking.
            )
            .map(|bir_file| BufReader::new(bir_file))
            .flat_map(|bir_file| bir::BirFileIterator::new(bir_file))
            .flat_map(|block| block.transactions);

        let max_address_id = {
            let db_connection = db_connection_pool.get()?;
            Address::max_id(&db_connection)?
        };

        if let Some(max_address_id) = max_address_id {
            let max_address_id = max_address_id as u64;
            let mut cluster_unifier = ClusterUnifier::new(max_address_id);
            cluster_unifier.unify_clusters_in_transactions(transactions);
            let cluster_representatives = cluster_unifier.into_cluster_representatives();
            save_cluster_representatives(db_connection_pool, &cluster_representatives)?;
        };

        info!("Finished ClusteringTask");

        Ok(())
    }

    fn get_indexes(&self) -> Vec<Index> {
        vec![]
    }
}

fn save_cluster_representatives(
    db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
    new_cluster_representatives: &[u64],
) -> Result<(), Error> {
    info!("Save cluster representatives");

    let current_cluster_representatives = load_all_cluster_representatives(db_connection_pool)?;

    let mut changed_cluster_representatives: Vec<(u64, u64)> = vec![];

    for (address_id, &new_cluster_representative) in new_cluster_representatives.iter().enumerate()
    {
        if new_cluster_representative != current_cluster_representatives[address_id] {
            changed_cluster_representatives.push((address_id as u64, new_cluster_representative))
        }
    }

    info!(
        "Update {} cluster representatives",
        changed_cluster_representatives.len()
    );

    let update_counter = Mutex::new(0);

    // TODO Handle inconsistency during updates in parallel transactions.
    changed_cluster_representatives
        .par_iter()
        .chunks(1_000_000)
        .for_each(|cluster_assignments| {
            let mut number_of_assignments = 0;

            // TODO Return error instead of panicking.
            let db_connection = db_connection_pool.get().unwrap();

            db_connection
                .transaction::<(), Error, _>(|| {
                    for (address_id, cluster_representative) in cluster_assignments {
                        let cluster_representative = *cluster_representative as i64;

                        // TODO Can `UnionJoin::find()` return `0`?
                        let cluster_representative = if cluster_representative > 0 {
                            Some(cluster_representative)
                        } else {
                            None
                        };

                        number_of_assignments += update_cluster_representative(
                            &db_connection,
                            *address_id as i64,
                            cluster_representative,
                        )?
                    }
                    Ok(())
                })
                .unwrap();

            let mut update_counter = update_counter.lock().unwrap();
            *update_counter += number_of_assignments;
            info!("Saved {} cluster representatives", update_counter);
        });

    Ok(())
}

// TODO Unify with in_memory_addres_map::load_all_addresses
fn load_all_cluster_representatives(
    db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<Vec<u64>, Error> {
    let max_id = {
        let db_connection = db_connection_pool.get()?;
        if let Some(max_id) = db_persistence::Address::max_id(&db_connection)? {
            max_id
        } else {
            0
        }
    };

    let limit = 100_000;

    let offsets: Vec<usize> = (0..(max_id + 1) as usize).collect();
    let offsets: Vec<usize> = offsets
        .chunks(limit)
        .map(|chunk| *chunk.first().unwrap())
        .collect();

    let cluster_assignment_chunks: Vec<Vec<ClusterAssignment>> = offsets
        .par_iter()
        .map(|offset| {
            // TODO Return error instead of panicking.
            let db_connection = db_connection_pool.get().unwrap();

            let chunk: Vec<ClusterAssignment> =
                ClusterAssignment::load_in_range(&db_connection, *offset as i64, limit as i64)
                    .unwrap();
            chunk
        })
        .collect();

    let cluster_assignments: Vec<ClusterAssignment> = cluster_assignment_chunks
        .into_iter()
        .flat_map(|chunk| chunk.into_iter())
        .collect();

    let mut cluster_representatives: Vec<u64> = vec![0; max_id as usize + 1];

    for cluster_assignment in cluster_assignments {
        cluster_representatives[cluster_assignment.id as usize] =
            cluster_assignment.cluster_representative.unwrap_or(0) as u64;
    }

    Ok(cluster_representatives)
}

fn update_cluster_representative(
    db_connection: &PgConnection,
    address_id: i64,
    cluster_representative: Option<i64>,
) -> Result<usize, diesel::result::Error> {
    diesel::update(
        schema::addresses::dsl::addresses.filter(schema::addresses::dsl::id.eq(address_id)),
    ).set(schema::addresses::dsl::cluster_representative.eq(cluster_representative))
        .execute(db_connection)
}
