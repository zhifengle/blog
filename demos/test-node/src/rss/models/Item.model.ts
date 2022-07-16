import { Column, Model, Table, HasOne, DataType } from 'sequelize-typescript';
import { Enclosure } from './Enclosure.model';

// 默认是 STRING ---> varchar(255)
// @Column({field: 'pub_date'})

@Table({ tableName: 'rss_items' })
export class Item extends Model {
  @Column
  link: string;
  @Column
  title: string;
  @Column
  guid?: string;
  @Column
  pubDate?: Date;
  @Column
  creator?: string;
  @Column(DataType.TEXT)
  summary?: string;
  @Column
  content?: string;
  @Column
  isoDate?: Date;
  @Column
  categories?: string;
  @Column
  contentSnippet?: string;
  @HasOne(() => Enclosure)
  enclosure?: Enclosure;
  @Column({ defaultValue: false })
  done: boolean;
  // magnet 的 hash. 必须填
  @Column({ allowNull: false })
  magnet: string;
}

// @Column({
//     type: DataType.FLOAT,
//     comment: 'Some value',
//   })
