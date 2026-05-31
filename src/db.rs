

struct DbHandler{
    conn: rusqlite::Connection
}
impl DbHandler{
    fn write_player(&self, player: PlayerRow) -> Result<()>{

    }
}