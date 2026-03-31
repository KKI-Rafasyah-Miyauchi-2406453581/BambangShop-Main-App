use dashmap:: DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber:: Subscriber;
pub struct SubscriberRepository;

// Singleton of Database
lazy_static! {
static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

impl SubscriberRepository {
}