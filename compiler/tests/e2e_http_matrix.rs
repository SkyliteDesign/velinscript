//! HTTP methods matrix + hello query + auth/role runtime (Gate 1).

#[path = "common/mod.rs"]
mod common;

use common::{http_get, http_request, spawn_generated_axum};

const SRC: &str = r#"
struct Item {
    title: string
}

struct Search {
    q: string
}

@GET("/ping")
fn ping(): string {
    return "ok";
}

@GET("/hello")
fn hello(name: string): string {
    return "Hello {name}";
}

@GET("/users/:id")
fn get_user(id: string): string {
    return "user-{id}";
}

@GET("/sum")
fn sum(a: int, b: int): string {
    return "n={a}";
}

@GET("/flag")
fn flag(on: bool): string {
    return "on={on}";
}

@GET("/search")
fn search(filter: Search): string {
    return "ok";
}

@GET("/f")
fn get_float(x: float): string {
    return "x={x}";
}

@POST("/items")
fn create_item(item: Item): Item {
    return item;
}

@POST("/tagged/:id")
fn tagged(id: string, item: Item): string {
    return "{id}";
}

@POST("/echo")
fn echo_msg(msg: string): string {
    return msg;
}

@PUT("/items/:id")
fn put_item(id: string, item: Item): Item {
    return item;
}

@PUT("/label")
fn put_label(msg: string): string {
    return msg;
}

@PATCH("/items/:id")
fn patch_item(id: string, item: Item): Item {
    return item;
}

@PATCH("/label")
fn patch_label(msg: string): string {
    return msg;
}

@DELETE("/items/:id")
fn delete_item(id: string): string {
    return "deleted-{id}";
}

@DELETE("/by-name")
fn del_by_name(name: string): string {
    return "bye-{name}";
}

@DELETE("/by-search")
fn del_search(filter: Search): string {
    return "ok";
}

@GET("/public")
fn public_ep(): string {
    return "pub-ok";
}

@GET("/secure")
@Auth
fn secure_ep(): string {
    return "sec-ok";
}

@GET("/admin")
@Role("admin")
fn admin_ep(): string {
    return "admin-ok";
}

@GET("/editor")
@Role("editor")
fn editor_ep(): string {
    return "editor-ok";
}

@GET("/either")
@Role("admin", "editor")
fn either_ep(): string {
    return "either-ok";
}
"#;

#[test]
fn e2e_http_auth_query_runtime_matrix() {
    let srv = spawn_generated_axum(SRC, "e2e_http_matrix", None);
    let port = srv.port;

    let (s, b) = http_get(port, "/ping", None);
    assert_eq!(s, 200, "GET /ping");
    assert!(b.contains("ok"), "ping body {}", b);

    let (s, b) = http_get(port, "/hello?name=Velin", None);
    assert_eq!(s, 200, "GET /hello?name=Velin");
    assert!(b.contains("Hello Velin"), "hello body {}", b);

    let (s, _) = http_get(port, "/hello", None);
    assert_eq!(s, 400, "GET /hello missing name");

    let (s, b) = http_get(port, "/users/42", None);
    assert_eq!(s, 200, "GET path");
    assert!(b.contains("user-42"), "path body {}", b);
    assert!(b.contains("user-42"), "path body {}", b);

    let (s, b) = http_get(port, "/sum?a=3&b=4", None);
    assert_eq!(s, 200, "GET int query");
    assert!(b.contains("n=3"), "sum body {}", b);

    let (s, b) = http_get(port, "/flag?on=true", None);
    assert_eq!(s, 200, "GET bool query");
    assert!(b.contains("on=true") || b.contains("on=true"), "flag {}", b);

    let (s, _) = http_get(port, "/search?q=velin", None);
    assert_eq!(s, 200, "GET query struct");

    let (s, b) = http_request(
        port,
        "POST",
        "/items",
        &[],
        Some(r#"{"title":"note"}"#),
    );
    assert_eq!(s, 200, "POST struct body");
    assert!(b.contains("note"), "post body {}", b);

    let (s, b) = http_request(
        port,
        "PUT",
        "/items/1",
        &[],
        Some(r#"{"title":"upd"}"#),
    );
    assert_eq!(s, 200, "PUT path+body");
    assert!(b.contains("upd"), "put body {}", b);

    let (s, b) = http_request(
        port,
        "PATCH",
        "/items/1",
        &[],
        Some(r#"{"title":"pat"}"#),
    );
    assert_eq!(s, 200, "PATCH path+body");
    assert!(b.contains("pat"), "patch body {}", b);

    let (s, b) = http_request(port, "DELETE", "/items/9", &[], None);
    assert_eq!(s, 200, "DELETE path");
    assert!(b.contains("deleted-9"), "delete body {}", b);

    let (s, b) = http_request(port, "POST", "/echo", &[], Some(r#""hi""#));
    assert_eq!(s, 200, "POST scalar json");
    assert!(b.contains("hi"), "echo {}", b);

    let (s, b) = http_get(port, "/f?x=1.5", None);
    assert_eq!(s, 200, "GET float query");
    assert!(b.contains("1.5"), "float body {}", b);

    let (s, b) = http_request(
        port,
        "POST",
        "/tagged/7",
        &[],
        Some(r#"{"title":"t"}"#),
    );
    assert_eq!(s, 200, "POST path+body");
    assert!(b.contains("7"), "tagged {}", b);

    let (s, _) = http_request(port, "POST", "/items", &[], Some("{"));
    assert_eq!(s, 400, "POST invalid json body");

    let (s, b) = http_request(port, "PUT", "/label", &[], Some(r#""put-s""#));
    assert_eq!(s, 200, "PUT scalar json");
    assert!(b.contains("put-s"), "put label {}", b);

    let (s, b) = http_request(port, "PATCH", "/label", &[], Some(r#""pat-s""#));
    assert_eq!(s, 200, "PATCH scalar json");
    assert!(b.contains("pat-s"), "patch label {}", b);

    let (s, b) = http_request(port, "DELETE", "/by-name?name=x", &[], None);
    assert_eq!(s, 200, "DELETE query scalar");
    assert!(b.contains("bye-x"), "delete query {}", b);

    let (s, _) = http_request(port, "DELETE", "/by-name", &[], None);
    assert_eq!(s, 400, "DELETE missing query");

    let (s, _) = http_request(port, "DELETE", "/by-search?q=z", &[], None);
    assert_eq!(s, 200, "DELETE query struct");

    let (s, b) = http_get(port, "/public", None);
    assert_eq!(s, 200, "public");
    assert!(b.contains("pub-ok"));

    let (s, _) = http_get(port, "/secure", None);
    assert_eq!(s, 401, "auth no header");
    let (s, b) = http_get(port, "/secure", Some("Bearer x"));
    assert_eq!(s, 200, "auth with header");
    assert!(b.contains("sec-ok"));

    let (s, _) = http_request(
        port,
        "GET",
        "/admin",
        &[("Authorization", "Bearer x")],
        None,
    );
    assert_eq!(s, 403, "admin no role");
    let (s, _) = http_request(
        port,
        "GET",
        "/admin",
        &[("Authorization", "Bearer x"), ("X-Role", "editor")],
        None,
    );
    assert_eq!(s, 403, "admin wrong role");
    let (s, b) = http_request(
        port,
        "GET",
        "/admin",
        &[("Authorization", "Bearer x"), ("X-Role", "admin")],
        None,
    );
    assert_eq!(s, 200, "admin ok");
    assert!(b.contains("admin-ok"));

    let (s, b) = http_request(
        port,
        "GET",
        "/editor",
        &[("Authorization", "Bearer x"), ("X-Role", "editor")],
        None,
    );
    assert_eq!(s, 200, "editor ok");
    assert!(b.contains("editor-ok"));
    let (s, _) = http_request(
        port,
        "GET",
        "/editor",
        &[("Authorization", "Bearer x"), ("X-Role", "admin")],
        None,
    );
    assert_eq!(s, 403, "editor does not accept admin (no global union)");

    let (s, b) = http_request(
        port,
        "GET",
        "/either",
        &[("Authorization", "Bearer x"), ("X-Role", "admin")],
        None,
    );
    assert_eq!(s, 200, "multi-role admin");
    assert!(b.contains("either-ok"));
    let (s, _) = http_request(
        port,
        "GET",
        "/either",
        &[("Authorization", "Bearer x"), ("X-Role", "editor")],
        None,
    );
    assert_eq!(s, 200, "multi-role editor");

    srv.kill();
}
