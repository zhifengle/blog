import {
  Column,
  Model,
  Table,
  ForeignKey,
  BelongsTo,
} from 'sequelize-typescript';
import { Item } from './Item.model';

@Table({ tableName: 'rss_enclosures' })
export class Enclosure extends Model {
  @Column
  url: string;
  @Column
  length?: number;
  @Column
  type?: string;
  @ForeignKey(() => Item)
  @Column
  itemId!: number;
  @BelongsTo(() => Item) item: Item;
}
