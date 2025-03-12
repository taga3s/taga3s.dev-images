import { Hono } from "hono";
import { basicAuth } from "hono/basic-auth";

type Bindings = {
	KV_TAGA3S_DEV_ASSETS: KVNamespace;
};

const v1 = new Hono<{ Bindings: Bindings }>();

v1.get("/work-history", async (c) => {
	const value = await c.env.KV_TAGA3S_DEV_ASSETS.get("work_history");
	if (!value) {
		return c.json({ work_history: [] });
	}

	const work_history = JSON.parse(value);

	return c.json({ work_history });
});

v1.put(
	"/work-history",
	basicAuth({
		username: "taga3s",
		password: "mameshiba1123",
	}),
	async (c) => {
		const { value } = await c.req.json();
		await c.env.KV_TAGA3S_DEV_ASSETS.put("work_history", JSON.stringify(value));
		return c.json({ message: "Work history updated" });
	},
);

v1.get("/works", async (c) => {
	const value = await c.env.KV_TAGA3S_DEV_ASSETS.get("works");
	if (!value) {
		return c.json({ works: [] });
	}

	const works = JSON.parse(value);

	return c.json({ works });
});

v1.put(
	"/works",
	basicAuth({
		username: "taga3s",
		password: "mameshiba1123",
	}),
	async (c) => {
		const { value } = await c.req.json();
		await c.env.KV_TAGA3S_DEV_ASSETS.put("works", JSON.stringify(value));
		return c.json({ message: "Works updated" });
	},
);

const app = new Hono();

app.route("/api/v1", v1);

export default app;
