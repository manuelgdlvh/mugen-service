pub trait MugenStatsReader {
    fn start<'a>(&self, fighter_one: &'a str, fighter_two: &'a str) -> anyhow::Result<&'a str>;
}