import { DataTypes, Model, Sequelize } from 'sequelize';

const sequelize = new Sequelize({
  dialect: 'sqlite',
  storage: 'database.sqlite',
});

export async function test() {
  sequelize.sync({ force: true });
  const User = sequelize.define(
    'user',
    { name: DataTypes.STRING },
    { timestamps: false }
  );
  const Task = sequelize.define(
    'task',
    { name: DataTypes.STRING },
    { timestamps: false }
  );
  const Tool = sequelize.define(
    'tool',
    {
      name: DataTypes.STRING,
      size: DataTypes.STRING,
    },
    { timestamps: false }
  );
  User.hasMany(Task);
  Task.belongsTo(User);
  User.hasMany(Tool, { as: 'Instruments' });
}
