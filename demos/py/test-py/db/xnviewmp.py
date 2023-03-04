from pathlib import Path
import sqlite3
import re


XNVIEW_DB_PATH = Path.home() / "AppData" / "Roaming" / "XnViewMP" / "xnview.db"


class XnViewDb:
    def __init__(self, db_path=XNVIEW_DB_PATH) -> None:
        self.conn = sqlite3.connect(db_path)
        self.cursor = self.conn.cursor()

    def create_or_get_tag_id(self, tag_name):
        tag_id = self.cursor.execute(
            "SELECT tagid FROM tags WHERE Label = ?", (tag_name,)
        ).fetchone()
        if tag_id:
            return tag_id[0]
        self.cursor.execute(
            "INSERT INTO tags (Label, ParentID, ID) VALUES (?,?,?)",
            (tag_name, -1, 0),
        )
        rowid = self.cursor.lastrowid
        self.conn.commit()
        return rowid

    def add_tag_to_image(self, tag_id, image_id):
        self.cursor.execute(
            "INSERT INTO TagsTree (ImageID, TagID) VALUES (?, ?)",
            (tag_id, image_id),
        )
        self.conn.commit()

    def add_tagstree(self, arr):
        self.cursor.executemany(
            "INSERT OR IGNORE INTO TagsTree (ImageID, TagID) VALUES (?, ?)",
            arr,
        )
        # self.conn.commit()

    # add tags for image's name starts with `yande.re`
    def add_tags_to_images(self, tag_name, image_name_prefix='yande.re'):
        tag_id = self.create_or_get_tag_id(tag_name)
        image_ids = self.cursor.execute(
            "SELECT id FROM images WHERE name LIKE ?",
            (f'{image_name_prefix}%',),
        ).fetchall()
        for image_id in image_ids:
            self.add_tag_to_image(tag_id, image_id[0])

    # add tags by image's name for yande.re
    def add_tags_to_images_by_name(self):
        image_name_prefix = 'yande.re '
        images = self.cursor.execute(
            "SELECT imageid,FileName FROM images WHERE FileName LIKE ? LIMIT 1",
            (f'{image_name_prefix}%',),
        ).fetchall()
        counter = 0
        for img in images:
            image_id = img[0]
            image_name = Path(img[1]).stem
            match = re.search(r'^yande.re \d+', image_name)
            if match:
                tags = re.split(r'\d+ ', image_name)[1].split(' ')
                tags = [tag.strip() for tag in tags if tag.strip() != '']
                arr = [(image_id, self.create_or_get_tag_id(tag)) for tag in tags]
                self.add_tagstree(arr)
                counter += 1
            if counter % 1000 == 0:
                self.conn.commit()
        
        self.conn.commit()


if __name__ == '__main__':
    db = XnViewDb()
    db.add_tags_to_images_by_name()
