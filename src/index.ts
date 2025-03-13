import { Hono } from "hono";
import { env } from "hono/adapter";
import { basicAuth } from "hono/basic-auth";

type Bindings = {
	r2_taga3s_dev_assets: R2Bucket;
};

const v1 = new Hono<{ Bindings: Bindings }>();

// Public routes

v1.get("/images/favs", async (c) => {
	return c.json({ images: [] });
});

v1.get("/work-history", async (c) => {
	const object = await c.env.r2_taga3s_dev_assets.get("json/work_history");
	if (!object) {
		return c.json({ work_history: [] });
	}

	const headers = new Headers();
	headers.set("Content-Type", "application/json");
	headers.set("etag", object.etag);

	return new Response(object.body, {
		headers,
	});
});

v1.get("/works", async (c) => {
	const object = await c.env.r2_taga3s_dev_assets.get("json/works");
	if (!object) {
		return c.json({ works: [] });
	}

	const headers = new Headers();
	headers.set("Content-Type", "application/json");
	headers.set("etag", object.etag);

	return new Response(object.body, {
		headers,
	});
});

// Admin routes

v1.use("/admin/*", async (c, next) => {
	const { BASIC_AUTH_USERNAME, BASIC_AUTH_PASSWORD } = env<{
		BASIC_AUTH_USERNAME: string;
		BASIC_AUTH_PASSWORD: string;
	}>(c);
	const middleware = basicAuth({
		username: BASIC_AUTH_USERNAME,
		password: BASIC_AUTH_PASSWORD,
	});
	return middleware(c, next);
});

v1.put("/admin/work-history", async (c) => {
	const { work_history } = await c.req.json();
	const object = await c.env.r2_taga3s_dev_assets.put(
		"json/work_history",
		JSON.stringify({ work_history }),
	);
	return c.json({ message: "Work history updated", path: object?.key });
});

v1.put("/admin/works", async (c) => {
	const { works } = await c.req.json();
	const object = await c.env.r2_taga3s_dev_assets.put(
		"json/works",
		JSON.stringify({ works }),
	);
	return c.json({ message: "Works updated", path: object?.key });
});

const app = new Hono();

app.route("/api/v1", v1);

export default app;
