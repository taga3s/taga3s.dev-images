import { Hono } from "hono";
import { env } from "hono/adapter";
import { basicAuth } from "hono/basic-auth";

type Bindings = {
	kv_taga3s_dev_assets: KVNamespace;
};

const v1 = new Hono<{ Bindings: Bindings }>();

// Public routes

v1.get("/work-history", async (c) => {
	const value = await c.env.kv_taga3s_dev_assets.get("work_history");
	if (!value) {
		return c.json({ work_history: [] });
	}

	const work_history = JSON.parse(value);

	return c.json({ work_history });
});

v1.get("/works", async (c) => {
	const value = await c.env.kv_taga3s_dev_assets.get("works");
	if (!value) {
		return c.json({ works: [] });
	}

	const works = JSON.parse(value);

	return c.json({ works });
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
	const { value } = await c.req.json();
	await c.env.kv_taga3s_dev_assets.put("work_history", JSON.stringify(value));
	return c.json({ message: "Work history updated" });
});

v1.put("/admin/works", async (c) => {
	const { value } = await c.req.json();
	await c.env.kv_taga3s_dev_assets.put("works", JSON.stringify(value));
	return c.json({ message: "Works updated" });
});

const app = new Hono();

app.route("/api/v1", v1);

export default app;
