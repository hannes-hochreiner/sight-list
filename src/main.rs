use ::sight_list::{
    get_js_id,
    objects::{
        contexts::{
            ListContext, ListEditContext, ListEditFieldContext, ListKmlContext, ListListContext,
            ListPageContext, MapContext, MapMarkerContext, MapView, SightContext, SightEditContext,
            SightEditFieldContext, SightListContext,
        },
        db::{List, Sight},
    },
    sight_list_error::SightListError,
};
use rocket::{delete, form::Form, fs::FileServer, get, launch, post, put, routes, FromForm};
use rocket_db_pools::{sqlx, Connection, Database};
use rocket_dyn_templates::Template;
use sqlx::Acquire;
use uuid::Uuid;

#[derive(Database)]
#[database("db")]
struct Db(sqlx::PgPool);

#[derive(FromForm, Debug)]
struct ListUpload {
    title: String,
    version: Uuid,
}

#[derive(FromForm, Debug)]
struct SightUpload {
    title: String,
    lat: f64,
    long: f64,
    description: Option<String>,
    version: Uuid,
    list_id: Uuid,
}

#[delete("/list/<id>")]
async fn list_delete(id: Uuid, mut db: Connection<Db>) -> Result<(), SightListError> {
    let mut trans = (&mut *db).begin().await?;

    sqlx::query("DELETE FROM lists WHERE id = $1;")
        .bind(&id)
        .execute(&mut *trans)
        .await?;

    trans.commit().await?;

    Ok(())
}

#[post("/list/<id>", data = "<list_upload>")]
async fn post_list(
    id: Uuid,
    list_upload: Form<ListUpload>,
    mut db: Connection<Db>,
) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;

    sqlx::query("INSERT INTO lists VALUES ($1, $2, $3);")
        .bind(&id)
        .bind(&Uuid::new_v4())
        .bind(&list_upload.title)
        .execute(&mut *trans)
        .await?;

    trans.commit().await?;

    lists(db).await
}

#[put("/list/<id>", data = "<list_upload>")]
async fn put_list(
    id: Uuid,
    list_upload: Form<ListUpload>,
    mut db: Connection<Db>,
) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;

    let res =
        sqlx::query("UPDATE lists SET version = $1, title = $2 WHERE id = $3 AND version = $4;")
            .bind(&Uuid::new_v4())
            .bind(&list_upload.title)
            .bind(&id)
            .bind(&list_upload.version)
            .execute(&mut *trans)
            .await?;

    if res.rows_affected() != 1 {
        return Err(SightListError::UpdateError);
    }

    trans.commit().await?;

    lists(db).await
}

#[post("/sight/<id>", data = "<sight_upload>")]
async fn post_sight(
    id: Uuid,
    sight_upload: Form<SightUpload>,
    mut db: Connection<Db>,
) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;

    sqlx::query("INSERT INTO sights VALUES ($1, $2, $3, $4, $5, $6, $7);")
        .bind(&id)
        .bind(&Uuid::new_v4())
        .bind(&sight_upload.title)
        .bind(&sight_upload.lat)
        .bind(&sight_upload.long)
        .bind(&sight_upload.description)
        .bind(&sight_upload.list_id)
        .execute(&mut *trans)
        .await?;

    trans.commit().await?;

    sight_list(sight_upload.list_id, db).await
}

#[delete("/sight/<id>")]
async fn sight_delete(id: Uuid, mut db: Connection<Db>) -> Result<(), SightListError> {
    let mut trans = (&mut *db).begin().await?;

    sqlx::query("DELETE FROM sights WHERE id = $1;")
        .bind(&id)
        .execute(&mut *trans)
        .await?;

    trans.commit().await?;

    Ok(())
}

#[put("/sight/<id>", data = "<sight_upload>")]
async fn put_sight(
    id: Uuid,
    sight_upload: Form<SightUpload>,
    mut db: Connection<Db>,
) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;
    let res = sqlx::query("UPDATE sights SET version = $1, title = $2, lat = $3, long = $4, description = $5, list_id = $6 WHERE id = $7 AND version = $8;")
        .bind(&Uuid::new_v4())
        .bind(&sight_upload.title)
        .bind(&sight_upload.lat)
        .bind(&sight_upload.long)
        .bind(&sight_upload.description)
        .bind(&sight_upload.list_id)
        .bind(&id)
        .bind(&sight_upload.version)
        .execute(&mut *trans)
        .await?;

    if res.rows_affected() != 1 {
        return Err(SightListError::UpdateError);
    }

    trans.commit().await?;

    sight_list(sight_upload.list_id, db).await
}

#[get("/")]
async fn index(mut db: Connection<Db>) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;
    let lists = sqlx::query_as::<_, List>("SELECT * FROM lists ORDER BY title;")
        .fetch_all(&mut *trans)
        .await?;

    Ok(Template::render(
        "index",
        ListListContext {
            lists: lists
                .into_iter()
                .map(|list| ListContext {
                    id: list.id,
                    title: list.title,
                })
                .collect(),
        },
    ))
}

#[get("/list")]
async fn lists(mut db: Connection<Db>) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;
    let lists = sqlx::query_as::<_, List>("SELECT * FROM lists ORDER BY title;")
        .fetch_all(&mut *trans)
        .await?;

    Ok(Template::render(
        "lists",
        ListListContext {
            lists: lists
                .into_iter()
                .map(|list| ListContext {
                    id: list.id,
                    title: list.title,
                })
                .collect(),
        },
    ))
}

#[get("/list/new")]
async fn list_new() -> Result<Template, SightListError> {
    Ok(Template::render(
        "list_edit",
        ListEditContext {
            id: Uuid::new_v4(),
            version: Uuid::new_v4(),
            fields: None,
        },
    ))
}

#[get("/list/<id>/edit")]
async fn list_edit(id: Uuid, mut db: Connection<Db>) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;
    let list = sqlx::query_as::<_, List>("SELECT * FROM lists WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *trans)
        .await?;

    Ok(Template::render(
        "list_edit",
        ListEditContext {
            id,
            version: list.version,
            fields: Some(ListEditFieldContext { title: list.title }),
        },
    ))
}

#[get("/list/<list_id>/sight/new")]
async fn sight_new(list_id: Uuid) -> Result<Template, SightListError> {
    let id = Uuid::new_v4();

    Ok(Template::render(
        "sight_edit",
        SightEditContext {
            list_id: list_id,
            id: id,
            js_id: get_js_id(&id),
            version: Uuid::new_v4(),
            fields: None,
        },
    ))
}

#[get("/sight/<id>")]
async fn sight_edit(id: Uuid, mut db: Connection<Db>) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;
    let sight = sqlx::query_as::<_, Sight>("SELECT * FROM sights WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *trans)
        .await?;

    Ok(Template::render(
        "sight_edit",
        SightEditContext {
            list_id: sight.list_id,
            id: id,
            js_id: get_js_id(&sight.id),
            version: sight.version,
            fields: Some(SightEditFieldContext {
                description: sight.description,
                lat: sight.lat,
                long: sight.long,
                title: sight.title,
            }),
        },
    ))
}

#[get("/list/<list_id>/sight")]
async fn sight_list(list_id: Uuid, mut db: Connection<Db>) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;
    let sights =
        sqlx::query_as::<_, Sight>("SELECT * FROM sights WHERE list_id = $1 ORDER BY title;")
            .bind(list_id)
            .fetch_all(&mut *trans)
            .await?;

    Ok(Template::render(
        "sight_list",
        SightListContext {
            sights: sights
                .into_iter()
                .map(|sight| SightContext {
                    id: sight.id,
                    js_id: get_js_id(&sight.id),
                    title: sight.title,
                    description: sight
                        .description
                        .and_then(|desc| Some(markdown::to_html(desc.as_str()))),
                    lat: sight.lat,
                    long: sight.long,
                })
                .collect(),
        },
    ))
}

#[get("/list/<list_id>")]
async fn list_page(list_id: Uuid, mut db: Connection<Db>) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;
    let list = sqlx::query_as::<_, List>("SELECT * FROM lists WHERE id = $1;")
        .bind(list_id)
        .fetch_one(&mut *trans)
        .await?;
    let sights =
        sqlx::query_as::<_, Sight>("SELECT * FROM sights WHERE list_id = $1 ORDER BY title;")
            .bind(list_id)
            .fetch_all(&mut *trans)
            .await?;
    let mut lat_min = f64::MAX;
    let mut lat_max = f64::MIN;
    let mut long_min = f64::MAX;
    let mut long_max = f64::MIN;
    let mut markers: Vec<MapMarkerContext> = Vec::new();
    let mut sight_ctxs: Vec<SightContext> = Vec::new();
    let num_of_sights = sights.len();

    for sight in sights {
        sight_ctxs.push(SightContext {
            id: sight.id,
            js_id: get_js_id(&sight.id),
            title: sight.title.clone(),
            description: sight
                .description
                .and_then(|desc| Some(markdown::to_html(desc.as_str()))),
            lat: sight.lat,
            long: sight.long,
        });
        markers.push(MapMarkerContext {
            js_id: get_js_id(&sight.id),
            lat: sight.lat,
            long: sight.long,
            content: sight.title,
        });
        lat_min = lat_min.min(sight.lat);
        lat_max = lat_max.max(sight.lat);
        long_min = long_min.min(sight.long);
        long_max = long_max.max(sight.long);
    }

    if num_of_sights == 0 {
        lat_min = 0.0;
        lat_max = 0.0;
        long_min = 0.0;
        long_max = 0.0;
    }

    Ok(Template::render(
        "list_page",
        ListPageContext {
            id: list.id,
            title: list.title,
            sights: sight_ctxs,
            map: MapContext {
                view: match lat_min == lat_max && long_min == long_max {
                    true => MapView::LatLongZoom {
                        lat: lat_min,
                        long: long_min,
                        zoom: 10,
                    },
                    false => MapView::Box {
                        lat_min: lat_min,
                        lat_max: lat_max,
                        long_min: long_min,
                        long_max: long_max,
                    },
                },
                markers,
            },
        },
    ))
}

#[get("/list/<list_id>/kml")]
async fn list_kml(list_id: Uuid, mut db: Connection<Db>) -> Result<Template, SightListError> {
    let mut trans = (&mut *db).begin().await?;
    let list = sqlx::query_as::<_, List>("SELECT * FROM lists WHERE id = $1;")
        .bind(list_id)
        .fetch_one(&mut *trans)
        .await?;
    let sights =
        sqlx::query_as::<_, Sight>("SELECT * FROM sights WHERE list_id = $1 ORDER BY title;")
            .bind(list_id)
            .fetch_all(&mut *trans)
            .await?;

    Ok(Template::render(
        "kml",
        ListKmlContext {
            title: list.title,
            sights: sights
                .into_iter()
                .map(|sight| SightContext {
                    id: sight.id,
                    js_id: get_js_id(&sight.id),
                    title: sight.title,
                    description: sight
                        .description
                        .and_then(|desc| Some(markdown::to_html(desc.as_str()))),
                    lat: sight.lat,
                    long: sight.long,
                })
                .collect(),
        },
    ))
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/static", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index,
                list_new,
                sight_delete,
                put_sight,
                sight_new,
                sight_edit,
                post_list,
                post_sight,
                sight_list,
                list_kml,
                list_page,
                lists,
                list_edit,
                put_list,
                list_delete
            ],
        )
        .attach(Template::fairing())
        .attach(Db::init())
}
