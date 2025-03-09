import { Hono } from "hono";

type Bindings = {
	BUCKET: R2Bucket;
};

const app = new Hono<{
	Bindings: Bindings;
}>();

app.get("/:id", async (c) => {
	const object = await c.env.BUCKET.get(c.req.param("id"));
	if (!object) {
		return c.notFound();
	}

	
	const body = await object.arrayBuffer();
	const contentType = object.httpMetadata?.contentType ?? "image/jpeg";

	return c.body(body, 200, {
		"Content-Type": contentType,
	});
});

export default app;
