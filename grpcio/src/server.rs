extern crate futures;
extern crate grpcio;
extern crate protos;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{ChannelBuilder, Environment, RpcContext, ServerBuilder, UnarySink};

use protos::rustserver::{Request, Response};
use protos::rustserver_grpc::{self, Rustserver};

#[derive(Clone)]
struct Service;

impl Rustserver for Service {
    fn process(&self, ctx: RpcContext, req: Request, sink: UnarySink<Response>) {
        println!("Received request {{ {:?} }}", req.get_original_file_name());
        let mut response = Response::new();
        
        // Simply byte rotation encryption
        // This is so we do some work and push the result
        let fek = b"\0";

        response.set_fek(fek.to_vec());
        response.set_econtent(req.get_content().iter().map(|&b| {
            b + fek[0]
        }).collect());

        let f = sink
            .success(response.clone())
            .map(move |_| println!("Encrypted with FEK={:?}", fek))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));

    // Build the channel args - this is passed is using an undocumented option on the ServerBuilder
    let throwaway_env = Arc::new(Environment::new(1));  // This won't get used - its simply so we can use a ChannelBuilder
    let channel_args = ChannelBuilder::new(throwaway_env)
        .max_receive_message_len(1024 * 1024 * 1024)
        .max_send_message_len(1024 * 1024 * 1024)
        .build_args();

    let service = rustserver_grpc::create_rustserver(Service);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .channel_args(channel_args)
        .bind("127.0.0.1", 50008)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    // This is just to hold the server open
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}