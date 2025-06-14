use std::convert::Infallible;
use std::fmt::Write;

use axum::extract::Request;
use axum::handler::Handler;
use axum::http::Method;
use axum::response::IntoResponse;
use axum::routing;
use tower_layer::Layer;
use tower_service::Service;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RouteInfo {
  pub method: Method,
  pub path: String,
}

pub struct Router<S> {
  pub root: axum::Router<S>,
  pub routes: Vec<RouteInfo>,
}

/// ```ignore
/// method!(get, Method::GET);
/// ```
///
/// Becomes:
///
/// ```ignore
/// fn get<T, H, L>(path: &str, handler: H) -> Router<S>
/// where
///   T: 'static,
///   H: Handler<T, S>,
/// {
///   route(Method::GET, path, handler, None::<()>)
/// }
///
/// fn get_with<T, H, L>(path: &str, handler: H, layer: L) -> Router<S>
/// where
///   T: 'static,
///   H: Handler<T, S>,
///   L: Layer<routing::Route> + Clone + Send + Sync + 'static,
///   L::Service: Service<Request> + Clone + Send + Sync + 'static,
///   <L::Service as Service<Request>>::Response: IntoResponse + 'static,
///   <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
///   <L::Service as Service<Request>>::Future: Send + 'static,
/// {
///   route(Method::GET, path, handler, Some(layer))
/// }
/// ```
macro_rules! method {
  ($name:ident, $name_with:ident, $method:expr) => {
    pub fn $name<T, H>(self, path: &str, handler: H) -> Self
    where
      T: 'static,
      H: Handler<T, S>,
    {
      self.route($method, path, handler, None::<()>)
    }

    pub fn $name_with<T, H, L>(self, path: &str, handler: H, layer: L) -> Self
    where
      T: 'static,
      H: Handler<T, S>,
      L: Layer<routing::Route> + Clone + Send + Sync + 'static,
      L::Service: Service<Request> + Clone + Send + Sync + 'static,
      <L::Service as Service<Request>>::Response: IntoResponse + 'static,
      <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
      <L::Service as Service<Request>>::Future: Send + 'static,
    {
      self.route($method, path, handler, Some(layer))
    }
  };
}

impl<S: Clone + Send + Sync + 'static> Router<S> {
  method!(connect, connect_with, Method::CONNECT);

  method!(delete, delete_with, Method::DELETE);

  method!(get, get_with, Method::GET);

  method!(head, head_with, Method::HEAD);

  method!(options, options_with, Method::OPTIONS);

  method!(patch, patch_with, Method::PATCH);

  method!(post, post_with, Method::POST);

  method!(put, put_with, Method::PUT);

  method!(trace, trace_with, Method::TRACE);

  pub fn new() -> Self {
    Self {
      root: axum::Router::new(),
      routes: Vec::new(),
    }
  }

  pub fn layer<L>(self, layer: L) -> Self
  where
    L: Layer<routing::Route> + Clone + Send + Sync + 'static,
    L::Service: Service<Request> + Clone + Send + Sync + 'static,
    <L::Service as Service<Request>>::Response: IntoResponse + 'static,
    <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
    <L::Service as Service<Request>>::Future: Send + 'static,
  {
    Self {
      root: self.root.layer(layer),
      routes: self.routes,
    }
  }

  pub fn nest(mut self, path: &str, other: Self) -> Self {
    self.root = self.root.nest(path, other.root);

    self.routes.extend(other.routes.into_iter().map(|route| RouteInfo {
      method: route.method,
      path: path.to_string() + &route.path,
    }));

    self
  }

  pub fn merge(mut self, other: Self) -> Self {
    self.root = self.root.merge(other.root);
    self.routes.extend(other.routes);

    self
  }

  fn route<T, H, L>(mut self, method: Method, path: &str, handler: H, layer: Option<L>) -> Self
  where
    T: 'static,
    H: Handler<T, S>,
    L: Layer<routing::Route> + Clone + Send + Sync + 'static,
    L::Service: Service<Request> + Clone + Send + Sync + 'static,
    <L::Service as Service<Request>>::Response: IntoResponse + 'static,
    <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
    <L::Service as Service<Request>>::Future: Send + 'static,
  {
    let handler = match method {
      Method::CONNECT => routing::connect(handler),
      Method::DELETE => routing::delete(handler),
      Method::GET => routing::get(handler),
      Method::HEAD => routing::head(handler),
      Method::OPTIONS => routing::options(handler),
      Method::PATCH => routing::patch(handler),
      Method::POST => routing::post(handler),
      Method::PUT => routing::put(handler),
      Method::TRACE => routing::trace(handler),
      _ => panic!("Unsupported HTTP method"),
    };

    let handler = if let Some(layer) = layer {
      handler.layer(layer)
    } else {
      handler
    };

    self.routes.push(RouteInfo {
      method,
      path: path.to_string(),
    });

    Self {
      root: self.root.route(path, handler),
      routes: self.routes,
    }
  }
}

impl<S> From<Router<S>> for routing::Router<S> {
  fn from(val: Router<S>) -> Self {
    val.root
  }
}

impl<S: Clone + Send + Sync + 'static> Default for Router<S> {
  fn default() -> Self {
    Self::new()
  }
}

pub fn print_routes(routes: &[RouteInfo]) {
  // sort routes by path (least specific to most specific)
  let mut routes = routes.to_vec();
  routes.sort_by(|a, b| a.path.cmp(&b.path));

  let mut output = String::new();

  for RouteInfo { method, path } in routes {
    let method = method.as_str();
    writeln!(output, "{method: >7} {path}").unwrap();
  }

  info!("routes:\n{}", output);
}
