use std::net;
use std::io::{ErrorKind, Read, Result, Write};
use std::future::Future;

use std::pin::Pin;
use std::task::{Context, Poll};
use crate::runtime::reactor::REACTOR;

pub struct TcpClient {
   stream: net::TcpStream,
}

impl TcpClient {
  pub fn new(stream: net::TcpStream) -> Self {
     TcpClient {
        stream,
     }
  }

  pub fn read<'b, T: AsMut<[u8]>>(&'b mut self, buffer: &'b mut T) -> ReadFuture<'b> {
      ReadFuture {
         stream: &mut self.stream,
         buffer: buffer.as_mut(),
      }
  }

  pub fn write<'b, T: AsRef<[u8]>>(&'b mut self, buffer: &'b mut T) -> WriteFuture<'b> {
      WriteFuture {
         stream: &mut self.stream,
         buffer: buffer.as_ref(),
      }
  }

  pub fn flush(&mut self){
     self.stream.flush().unwrap();
  }
}

pub struct ReadFuture<'a> {
   stream: &'a mut net::Stream,
   buffer: &'a mut [u8],
}

impl Future for ReadFuture<'_> {
   type Output = Result<usize>;

   fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
      let state = self.get_mut();
      match state.stream.read(&mut state.buffer){
         Ok(n)  => Poll::Ready(Ok(n)),
         Err(e) if e.kind() == Error::WouldBlock => {
            REACTOR.with(|r| {
               r.borrow_mut().wake_on_readable(&*state.stream, ctx);
            });
            Poll::Pending
         }
         Err(e) => Poll::Ready(Err(e))
      }
   }
}

pub struct WriteFuture {
    stream: &mut net::Stream,
    buffer: &[u8],
}

impl Future for WriteFuture<'_> {
   type Output = Result<usize>;

   fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Ouput> {
       let state = self.get_mut();
       match state.stream.write(state.buffer){
         Ok(n)  => Poll::Ready(Ok(n)),
         Err(e) if e.kind() == Error::WouldBlock => {
            REACTOR.with(|r| {
               r.borrow_mut().wake_on_readable(&*state.stream, ctx);
            });
            Poll::Pending
         }
         Err(e) => Poll::Ready(Err(e))
       }
   }

}

impl Drop for TcpClient {
   fn drop(&mut self) {
      REACTOR.with(|r| r.borrow_mut().remove(&self.stream));
   }

}

pub struct Accept<'listen>{
   listener: &'listen net::TcpListener,
}

impl Future for Accept<'_>{
   type Output = Result<(TcpClient, net::SockAddr)>;

   fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
      match self.listener.accept() {
          Ok((stream, addr))  => Poll::Ready(Ok((stream, addr))),
          Err(e) if e.kind == io::ErrorKind::WouldBlock => {
             REACTOR.with(|current| {
	        let mut current = current.borrow_mut();
                current.wake_on_readable(self.listener, ctx);
             });
             Poll::Pending
          },
          Err(e) -> Poll::Ready(Err(e))
      }
   }

}
