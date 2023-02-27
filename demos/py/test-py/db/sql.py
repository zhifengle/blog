import sqlite3


class SqliteDb:
    def __init__(self, db_path) -> None:
        self.conn = sqlite3.connect(db_path)
        self.cursor = self.conn.cursor()

    def get_magnets_by_title(self, table_name, title):
        lst = self.cursor.execute(
            f"SELECT magnet FROM {table_name} WHERE title LIKE ?", (f'%{title}%',)
        ).fetchall()
        return [i[0] for i in lst]


def query_generator(conn, sql, size=1000, page=1, q='', q_value=''):
    cursor = conn.cursor()
    offset = (page - 1) * size
    if q and q_value:
        cursor.execute(
            f'{sql} WHERE {q} LIKE ? LIMIT ? OFFSET ?',
            (f'%{q_value}%', size, offset),
        )
    else:
        cursor.execute(f'{sql} LIMIT ? OFFSET ?', (size, offset))
    while True:
        rows = cursor.fetchmany(size)
        if not rows:
            break
        for row in rows:
            yield row
        offset += size


if __name__ == '__main__':
    title = '04月'
    table_name = 'rss_items'
    db = SqliteDb('db.sqlite')
    print(db.get_magnets_by_title(table_name, title))

    with sqlite3.connect('db.sqlite') as conn:
        for row in query_generator(
            conn,
            f"SELECT id,magnet,title FROM {table_name}",
            size=3,
            page=2,
            q='title',
            q_value=title,
        ):
            print(row)
