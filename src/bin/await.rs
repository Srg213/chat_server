use tokio::time::Duration;

async fn sleep_then_print(timer: i32) {
    println!("Start timer {}.", timer);

    tokio::time::sleep(Duration::from_secs(1)).await;
    //                                            ^ execution can be paused here
    // If no await, then task are run without any switching as swap can only happen at an .await
    // await will context-switch here

    println!("Timer {} done.", timer);
}

// By running all async expressions on the current task, the expressions are
// able to run **concurrently** but not in **parallel**. This means all
// expressions are run on the same thread and if one branch blocks the thread,
// all other expressions will be unable to continue. If parallelism is
// required, spawn each async expression using [`tokio::spawn`] and pass the
// join handle to `join!`.
// async transforms a block of code into a state machine that implements a trait called Future.
#[tokio::main]
async fn main() {
    // The join! macro lets you run multiple things concurrently.
    // Concurrently vs parrallel-
    // Concurrency is on same thread, i.e. context-switching on a single processing unit
    // Parrallelism - uses mutliple threads at once, multi threading CPU
    tokio::join!(
        sleep_then_print(1),
        sleep_then_print(2),
        sleep_then_print(3),
    );
}

// Async code should never spend a long time without reaching an .await.

