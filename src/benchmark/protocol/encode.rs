use bytes::Bytes;

pub fn generate_get_command(key: String) -> Bytes {
    let body = generate_get_request_body(key);
    let header = format!("POST / HTTP/1.1\r\nContent-Type: application/json\r\nConnection: keep-alive\r\nContent-Length: {}\r\n\r\n", body.len());
    let request = [Bytes::from(header), body].concat();
    Bytes::from(request)
}

pub fn generate_set_command(key: String, value: String) -> Bytes {
    let body = generate_set_request_body(key, value);
    let header = format!("POST / HTTP/1.1\r\nContent-Type: application/json\r\nConnection: keep-alive\r\nContent-Length: {}\r\n\r\n", body.len());
    let request = [Bytes::from(header), body].concat();
    Bytes::from(request)
}

fn generate_get_request_body(key: String) -> Bytes {
    Bytes::from(format!("{{\"command\":\"GET\",\"key\":\"{}\"}}", key))
}

fn generate_set_request_body(key: String, value: String) -> Bytes {
    let body = [
        Bytes::from(format!("{{\"command\":\"SET\",\"key\":\"{}\",\"value\":\"", key)),
        Bytes::from(value),
        Bytes::from(format!("\",\"id\":{}}}", 1))
    ].concat();
    return Bytes::from(body);
}