use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

const TEXTO_CRIACAO: &str = "CREATE TABLE IF NOT EXISTS versao(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    versao REAL NOT NULL,
    download VARCHAR(255)
)";

const SELECT_VERSOES: &str =
    "SELECT id, versao, download FROM versao ORDER BY versao DESC LIMIT 10";

#[derive(Serialize, Debug, Deserialize)]
pub struct VersaoId {
    pub id: i32,
    pub versao: f32,
    pub download: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Versao {
    pub id: i32,
    pub versao: f32,
    pub download: String,
}

pub fn adquirir_conexao() -> Result<Connection, rusqlite::Error> {
    let conexao = Connection::open("database.db")?;

    criar_db(&conexao);
    Ok(conexao)
}

fn criar_db(conexao: &Connection) {
    conexao.execute(TEXTO_CRIACAO, ());
}

pub fn adquirir_versoes(conexao: &Connection) -> Result<Vec<Versao>, rusqlite::Error> {
    let mut stmt = conexao.prepare(SELECT_VERSOES)?;
    let versao_iter = stmt.query_map([], |row| {
        Ok(Versao {
            id: row.get(0)?,
            versao: row.get(1)?,
            download: row.get(2)?,
        })
    })?;
    let versoes: Vec<_> = versao_iter
        .into_iter()
        .filter_map(|versao| versao.ok())
        .collect();
    Ok(versoes)
}

pub fn adicionar_versao(conexao: &Connection, versao: &Versao) -> Result<(), rusqlite::Error> {
    conexao.execute(
        "INSERT INTO versao (versao, download) VALUES (?, ?)",
        params![versao.versao, versao.download],
    )?;
    Ok(())
}

pub fn remover_versao(conexao: &Connection, id_versao: i32) -> Result<(), rusqlite::Error> {
    conexao.execute("DELETE FROM versao WHERE id = ?", params![id_versao])?;
    Ok(())
}

pub fn modificar_versao(conexao: &Connection, versao: &VersaoId) -> Result<(), rusqlite::Error> {
    conexao.execute(
        "UPDATE versao SET versao = ?, download = ? WHERE id = ?",
        params![versao.versao, versao.download, versao.id],
    );
    Ok(())
}
