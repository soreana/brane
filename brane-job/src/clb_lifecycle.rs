use crate::errors::JobError;
use crate::interface::{Callback, CallbackKind, Event, EventKind};
use anyhow::Result;

/* TIM */
/// **Edited: added doc comments and now returning a JobError.**
/// 
/// Handles an incoming lifecycle message, which basically just passes the callback as an event.
/// 
/// **Arguments**
///  * `callback`: The callback message we received, already parsed into a struct.
/// 
/// **Returns**  
/// A list of events to fire on success, or else a JobError listing what went wrong.
pub fn handle(callback: Callback) -> Result<Vec<(String, Event)>, JobError> {
    let job_id = callback.job.clone();
    let application = callback.application.clone();
    let location_id = callback.location.clone();
    let order = callback.order;

    // Switch on the kind to map it to an EventKind
    let kind = match &callback.kind() {
        CallbackKind::Unknown => {
            debug!("Received Unkown callback: {:?}", callback);
            return Ok(vec![]);
        }
        CallbackKind::Ready => EventKind::Ready,
        CallbackKind::Initialized => EventKind::Initialized,
        CallbackKind::Started => EventKind::Started,
        CallbackKind::Heartbeat => panic!("Encountered a Heartbeat callback in a non-heartbeat callback handler; this should never happen!"),
        CallbackKind::Finished => EventKind::Finished,
        CallbackKind::Stopped => EventKind::Stopped,
        CallbackKind::Failed => EventKind::Failed,
    };

    // Construct the new event
    let key = format!("{}#{}", job_id, order);
    let payload = callback.payload;
    let category = String::from("job");
    let event = Event::new(
        kind,
        job_id,
        application,
        location_id,
        category,
        order as u32,
        Some(payload),
        None,
    );

    // Done!
    Ok(vec![(key, event)])
}
/*******/
