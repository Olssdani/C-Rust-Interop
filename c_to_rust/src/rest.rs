use actix_web::{web::{self, Data}, App, HttpResponse, HttpServer};
use once_cell::sync::Lazy;
use std::{
    sync::mpsc::{self, Receiver, Sender}, thread::{self, JoinHandle}
};

static mut INTERNAL: Lazy<InternalThreadData> = Lazy::new(|| InternalThreadData::default());

#[derive(Default)]
struct InternalThreadData {
    thread_handle: Option<JoinHandle<()>>,
    receiver: Option<Receiver<()>>,
}

#[actix_web::get("/health")]
async fn health(data: web::Data<Sender<()>>) -> HttpResponse {
    data.send(()).unwrap();
    HttpResponse::Ok().finish()
}

#[no_mangle]
pub unsafe extern "C" fn start_web() -> bool {
    if let Err(error) = internal_start_web() {
        println!("{}", error);
        return false;
    }

    true
}

unsafe fn internal_start_web() -> Result<(), String> {
    println!("Starting up web server");
    let (sender, receiver) = mpsc::channel();

    INTERNAL.receiver = Some(receiver);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build().map_err(|_| "Failed to build runtime".to_string())?;

    INTERNAL.thread_handle = Some(thread::spawn(move || {
        rt.block_on(async move {
            let server = HttpServer::new(move || {
                App::new()
                .app_data(Data::new(sender.clone()))
                    .service(health)
                    .default_service(web::route().to(HttpResponse::NotFound))
            })
            .bind(("127.0.0.1", 8080))
            .unwrap();
        
            server.run()
            .await.unwrap();
        })
    }));

    Ok(())
}


#[no_mangle]
pub unsafe extern "C" fn has_health() -> bool {
    if let Some(receiver) = &INTERNAL.receiver {
        return receiver.try_recv().is_ok();
    }
    false
}
