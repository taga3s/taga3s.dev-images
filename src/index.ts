import { Hono } from "hono";
import work_experiences from "../assets/data/work_experiences.json";
import works from "../assets/data/works.json";

const v1 = new Hono();

v1.get("/work-experiences", async (c) => {
	return c.json({ work_experiences });
});

v1.get("/works", async (c) => {
	return c.json({ works });
});

type Bindings = {};

const app = new Hono<{
	Bindings: Bindings;
}>();

app.route("/api/v1", v1);

export default app;
