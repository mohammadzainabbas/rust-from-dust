// ZOMBIES (checklist when writing tests)
// Link: https://youtu.be/0_UttFDnV3k?t=3539
//
// Anything that takes a sequence (vector, array etc), test 'em with ZOM
//
// Z - zero
// O - one
// M - many
// B - boundary conditions
// I - interface
// E - exceptions
// S - simple scenarios

#[tokio::test]
async fn test_todo() -> Result<(), anyhow::Error> {
    Ok(())
}

#[tokio::test]
async fn test_say_hello_default() -> Result<(), anyhow::Error> {
    let mut routers = get_routers().await.into_service();
    let uris = vec!["/hello", "/hello?names=Mohammad", "/hello?"];

    for uri in uris {
        let request = Request::builder().uri(uri).body(Body::empty())?;
        let response = ServiceExt::<Request<Body>>::ready(&mut routers)
            .await?
            .call(request)
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        assert_eq!(&body[..], b"<h3> Hello World! </h3>");
    }

    Ok(())
}

#[tokio::test]
async fn test_say_hello() -> Result<(), anyhow::Error> {
    let mut routers = get_routers().await.into_service();
    let uris_res = vec![
        ("/hello?name=", ""),
        ("/hello?name=Mohammad", "Mohammad"),
        ("/hello?name=1234", "1234"),
        ("/hello?name=Mohammad1234", "Mohammad1234"),
    ];

    for (uri, res) in uris_res {
        let request = Request::builder().uri(uri).body(Body::empty())?;
        let response = ServiceExt::<Request<Body>>::ready(&mut routers)
            .await?
            .call(request)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        assert_eq!(&body[..], format!("<h3> Hello {res}! </h3>").as_bytes());
    }

    Ok(())
}

#[tokio::test]
async fn test_say_path() -> Result<(), anyhow::Error> {
    let mut routers = get_routers().await.into_service();

    let uri = "/hello/";
    let request = Request::builder().uri(uri).body(Body::empty())?;
    let response = ServiceExt::<Request<Body>>::ready(&mut routers)
        .await?
        .call(request)
        .await?;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let uris_res = vec![
        ("/hello/what", "what"),
        ("/hello/Mohammad", "Mohammad"),
        ("/hello/1234", "1234"),
        ("/hello/name=Mohammad123", "name=Mohammad123"),
    ];

    for (uri, res) in uris_res {
        let request = Request::builder().uri(uri).body(Body::empty())?;
        let response = ServiceExt::<Request<Body>>::ready(&mut routers)
            .await
            .unwrap()
            .call(request)
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        assert_eq!(&body[..], format!("<h3> Hello {res}! </h3>").as_bytes());
    }

    Ok(())
}
