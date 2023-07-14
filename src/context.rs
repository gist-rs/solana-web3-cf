pub fn get_maybe_env_value(ctx: RouteContext<()>, binding: &str) -> Option<String> {
    ctx.env.var(binding).ok().map(|value| value.to_string())
}
