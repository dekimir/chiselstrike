use crate::framework::prelude::*;

async fn run_add_and_load_test(c: TestContext) {
    c.chisel.write(
        "models/dated.ts",
        r#"
        import { ChiselEntity } from "@chiselstrike/api"
        export class Dated extends ChiselEntity {
            date: Date;
        }"#,
    );
    c.chisel.write(
        "routes/read.ts",
        r#"
        import { Dated } from "../models/dated.ts";
        export default async function chisel(req: Request) {
            let dated = (await Dated.findOne({}))!;
            return dated.date.toUTCString();
        }
        "#,
    );
    c.chisel.apply_ok().await;
    c.chisel.post_json("/dev/store", json!({})).await;
    c.chisel
        .get("/dev/read")
        .send()
        .await
        .assert_text("Thu, 01 Sep 2022 12:13:14 GMT");
}

#[chisel_macros::test(modules = Deno)]
pub async fn create_and_load(c: TestContext) {
    c.chisel.write(
        "routes/store.ts",
        r#"
        import { Dated } from "../models/dated.ts";
        export default async function chisel(req: Request) {
            await Dated.create({
                date: new Date(Date.parse('01 Sep 2022 12:13:14 GMT')),
            });
        }
        "#,
    );
    run_add_and_load_test(c).await;
}

#[chisel_macros::test(modules = Deno)]
pub async fn build_save_and_load(c: TestContext) {
    c.chisel.write(
        "routes/store.ts",
        r#"
        import { Dated } from "../models/dated.ts";
        export default async function chisel(req: Request) {
            const d = await Dated.build({
                date: new Date(Date.parse('01 Sep 2022 12:13:14 GMT')),
            });
            await d.save();
        }
        "#,
    );
    run_add_and_load_test(c).await;
}

#[chisel_macros::test(modules = Deno)]
pub async fn insert_and_load(c: TestContext) {
    c.chisel.write(
        "routes/store.ts",
        r#"
        import { Dated } from "../models/dated.ts";
        export default async function chisel(req: Request) {
            const correct_date = new Date(Date.parse('01 Sep 2022 12:13:14 GMT'));
            const false_date = new Date(Date.parse('01 Jan 2000 10:00:00 GMT'));
            const d = await Dated.upsert({
                restrictions: {},
                create: { date: correct_date },
                update: { date: false_date }
            });
            await d.save();
        }
        "#,
    );
    run_add_and_load_test(c).await;
}

#[chisel_macros::test(modules = Deno)]
pub async fn update_and_load(c: TestContext) {
    c.chisel.write(
        "routes/store.ts",
        r#"
        import { Dated } from "../models/dated.ts";
        export default async function chisel(req: Request) {
            const correct_date = new Date(Date.parse('01 Sep 2022 12:13:14 GMT'));
            const false_date = new Date(Date.parse('01 Jan 2000 10:00:00 GMT'));
            await Dated.create({
                date: false_date,
            });
            const d = await Dated.upsert({
                restrictions: {},
                create: { date: false_date },
                update: { date: correct_date }
            });
        }
        "#,
    );
    run_add_and_load_test(c).await;
}

#[chisel_macros::test(modules = Deno)]
pub async fn crud(c: TestContext) {
    c.chisel.write(
        "models/dated.ts",
        r#"
        import { ChiselEntity } from "@chiselstrike/api"
        export class Dated extends ChiselEntity {
            date: Date;
        }"#,
    );
    c.chisel.write(
        "routes/dates.ts",
        r#"
        import { Dated } from "../models/dated.ts";
        export default Dated.crud();
        "#,
    );
    c.chisel.write(
        "routes/read.ts",
        r#"
        import { Dated } from "../models/dated.ts";
        export default async function chisel(req: Request) {
            let dated = (await Dated.findOne({}))!;
            return dated.date.toUTCString();
        }
        "#,
    );
    c.chisel.apply_ok().await;
    c.chisel
        .post_json(
            "/dev/dates",
            json!({
                "date": 1662624988000i64
            }),
        )
        .await;
    assert_eq!(
        c.chisel.get_text("/dev/read").await,
        "Thu, 08 Sep 2022 08:16:28 GMT"
    );
    json_is_subset(
        &c.chisel.get_json("/dev/dates?all=true").await,
        &json!({
            "results": [{
                "date": 1662624988000i64
            }]
        }),
    )
    .unwrap();

    json_is_subset(
        &c.chisel.get_json("/dev/dates?.date=1662624988000").await,
        &json!({
            "results": [{
                "date": 1662624988000i64
            }]
        }),
    )
    .unwrap();

    json_is_subset(
        &c.chisel.get_json("/dev/dates?.date=1662624988000.0").await,
        &json!({
            "results": [{
                "date": 1662624988000i64
            }]
        }),
    )
    .unwrap();
}

#[chisel_macros::test(modules = Deno)]
pub async fn filtering(c: TestContext) {
    c.chisel.write(
        "models/dated.ts",
        r#"
        import { ChiselEntity } from "@chiselstrike/api"
        export class Dated extends ChiselEntity {
            date: Date;
        }"#,
    );
    c.chisel.write(
        "routes/store.ts",
        r#"
        import { Dated } from "../models/dated.ts";
        export default async function chisel(req: Request) {
            await Dated.create({
                date: new Date(Date.parse('01 Sep 2022 12:13:14 GMT')),
            });
        }
        "#,
    );
    c.chisel.write(
        "routes/filter.ts",
        r#"
        import { Dated } from "../models/dated.ts";
        export default async function chisel(req: Request) {
            const dated = await Dated.findOne({
                date: new Date(Date.parse('01 Sep 2022 12:13:14 GMT')),
            });
            return dated!.date.toUTCString();
        }
        "#,
    );
    c.chisel.apply_ok().await;
    c.chisel.post("/dev/store").send().await.assert_ok();
    c.chisel
        .get("/dev/filter")
        .send()
        .await
        .assert_text("Thu, 01 Sep 2022 12:13:14 GMT");
}
