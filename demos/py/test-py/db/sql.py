import sqlite3


class SqliteDb:
    def __init__(self, db_path) -> None:
        self.conn = sqlite3.connect(db_path)
        self.cursor = self.conn.cursor()
    def get_magnets_by_title(self, table_name ,title):
        lst = self.cursor.execute(f"SELECT magnet FROM {table_name} WHERE title LIKE ?", (f'%{title}%',)).fetchall()
        return [i[0] for i in lst]
    # close db

if __name__ == '__main__':
    title = '[NC-Raws]'
    table_name = 'rss_items'
    db = SqliteDb('db.sqlite')
    print(db.get_magnets_by_title(table_name, title))

    with sqlite3.connect('db.sqlite') as conn:
        cursor = conn.cursor()
        cursor.execute("SELECT magnet FROM rss_items WHERE title LIKE ?", (f'%{title}%',)).fetchall()
        # cursor.execute("SELECT magnet FROM rss_items WHERE title LIKE ?", (f'%{title}%',)).fetchone()
        # cursor.execute("SELECT magnet FROM rss_items WHERE title LIKE ?", (f'%{title}%',)).fetchmany(3)
        # cursor.execute("SELECT magnet FROM rss_items WHERE title LIKE ?", (f'%{title}%',)).fetch