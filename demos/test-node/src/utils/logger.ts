import moment from 'moment-timezone';
import path from 'path';
import { createLogger, format, transports } from 'winston';

const { combine, timestamp, label, printf } = format;

const myFormat = printf(({ level, message, label, timestamp }) => {
  return `${timestamp} [${label}] ${level}: ${message}`;
});

const timezoned = () =>
  moment().tz('Asia/Shanghai').format('YYYY-MM-DD HH:mm:ss');

const homedir = require('os').homedir();
export function loggerFactory(
  name: string = 'my-logger',
  logsPath: string = homedir
) {
  const logger = createLogger({
    level: 'info',
    format: combine(
      label({ label: name }),
      timestamp({ format: timezoned }),
      myFormat
    ),
    defaultMeta: { service: `${name}-app` },
    transports: [
      new transports.File({
        filename: path.join(logsPath, `${name}-error.log`),
        level: 'error',
      }),
      new transports.File({
        filename: path.join(logsPath, `${name}-combined.log`),
      }),
    ],
  });

  // logger.add(
  //   new transports.Console({
  //     format: format.simple(),
  //   })
  // );
  // if (process.env.NODE_ENV !== 'production') {
  //   logger.add(
  //     new transports.Console({
  //       format: format.combine(format.colorize(), format.simple()),
  //     })
  //   );
  // }
  logger.add(new transports.Console());
  return logger;
}
