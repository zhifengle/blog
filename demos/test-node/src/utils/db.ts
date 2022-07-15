import { DataTypes, Model, Sequelize } from 'sequelize';

const sequelize = new Sequelize({
  dialect: 'sqlite',
  storage: 'database.sqlite',
});

export async function test() {
  (async () => {
    await sequelize.sync();
    const jane = await User.create({ name: 'Jane', age: 100, cash: 5000 });
    await jane.increment({
      age: 2,
      cash: 100,
    });
  })();
}
class User extends Model {
  declare id: number;
  name: string;
  age: number;
  cash: number;
}

User.init(
  {
    name: DataTypes.TEXT,
    favoriteColor: {
      type: DataTypes.TEXT,
      defaultValue: 'green',
    },
    age: DataTypes.INTEGER,
    cash: DataTypes.INTEGER,
  },
  { sequelize }
);
