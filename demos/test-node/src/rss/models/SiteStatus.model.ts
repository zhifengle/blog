import { Column, Model, Table } from 'sequelize-typescript';

@Table({ tableName: 'sites_status' })
export class SiteStatus extends Model {
  @Column
  name!: string;
  @Column({ defaultValue: false })
  needLogin: boolean;
  @Column({ defaultValue: false })
  abnormalOp: boolean;
}
