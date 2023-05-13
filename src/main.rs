mod queue_lib {
    pub mod queue;
    pub mod producer;
    pub mod consumer;
    mod daemon;
    mod worker;
}

mod http_comms_lib {
    mod http_request_extracter;
    pub mod http_request_handler;
}

mod pub_sub_lib {
    pub mod shared_memory;
    pub mod queue_manager;
    pub mod subscription_manager;
    pub mod topic_manager;
}

use std::sync::Arc;
use http_comms_lib::http_request_handler::{HttpRequestHandler, HttpRequestHandlerWrapper};
use pub_sub_lib::{shared_memory::SharedMemory, topic_manager::TopicManager, subscription_manager::SubscriptionManager, queue_manager::QueueManager};

fn main() {
    let shared_memory: Arc<SharedMemory> = Arc::new(SharedMemory::new());
    let topic_manager: Arc<TopicManager> = Arc::new(TopicManager::new(
        Arc::clone(&shared_memory)
    ));
    let queue_manager: Arc<QueueManager> = Arc::new(QueueManager::new(
        Arc::clone(&shared_memory)
    ));
    let subscription_manager: Arc<SubscriptionManager> = Arc::new(SubscriptionManager::new(
        Arc::clone(&shared_memory),
        Arc::clone(&queue_manager)
    ));
    let http_request_handler: Arc<HttpRequestHandler> = Arc::new(HttpRequestHandler::new(
        topic_manager, 
        subscription_manager, 
        Arc::clone(&queue_manager)
    ));
    let http_request_handler_wrapper: HttpRequestHandlerWrapper = HttpRequestHandlerWrapper::new(
        Arc::clone(&http_request_handler)
    );
    http_request_handler_wrapper.run();
}
