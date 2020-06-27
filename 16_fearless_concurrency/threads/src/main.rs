use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
  // thread1();
  // thread2();
  // thread3();
  // thread4();
  // thread5();
  // thread6();
  // thread7();
  // thread8();
  dead_lock();
}

fn thread1() {
  // This thread gets killed when the main thread exits
  thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
}

fn thread2() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  // This forces the parent thread to wait for the child thread to finish
  handle.join().unwrap();
}

fn thread3() {
  let v = vec![1, 2, 3];

  // To share data between threads, we use `move` to give ownership of the variable to the thread.
  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });
  // But v is not accessible anymore here.

  handle.join().unwrap();
}

fn thread4() {
  // Alternatively threads can communicate via messages.
  let (tx, rx) = mpsc::channel();

  // The thread gets ownership of the transmitter
  thread::spawn(move || {
    let val = String::from("hi");

    // send returns a Result
    tx.send(val).unwrap();

    // We can not access val after sending it
  });

  // Here we receive messages
  let received = rx.recv().unwrap();
  println!("Got: {}", received);

  /*
    recv blocks and waits for a message, it returns a Result as well.
    An error in thrown if the channel closes without getting a message.

    try_recv does not block,
    it returns a Result instantly telling us if there has been a message
    or an error if there is no error.
  */
}

fn thread5() {
  let (tx, rx) = mpsc::channel();

  // We can send multiple messages
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}

// MPSC means Multiple Producers Single Consumer, meaning we can have multiple senders but just one receiver.
fn thread6() {
  let (tx, rx) = mpsc::channel();
  // We can clone the transmitter to send data from multiple places
  let tx1 = mpsc::Sender::clone(&tx);

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}

// Alternatively we can also share a state between threads and use Mutexes.
use std::sync::Mutex;
fn thread7() {
  let m = Mutex::new(5);

  {
    // lock blocks the thread until it can acquire the mutex
    let mut num = m.lock().unwrap();

    // Then the value can be modified
    *num = 6;
  }
  println!("m = {:?}", m);
  // When the mutex goes out of scope, the data it contains gets freed
}

// When we have multiple threads we can give ownership of an object too all of them. We need to use Arc that is atomic and thread-safe but slower than Rc.
use std::sync::Arc;
fn thread8() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}

/*
  Any object implementing the trait Send can gets its ownership transfered to a thread.
  Any object implementing the trait Sync can be accessed in threads.
  If an object has only properties that implement those 2 traits, the object itself implements automatically those 2 traits.
*/