import { dayjs } from './index';

/** 格式化时间 */
export function format(timestamp: number): string {
  const time = dayjs(timestamp);
  const now = dayjs();
  if (now.isSame(time, 'day')) {
    return time.format('H:m');
  }
  if (now.isSame(time, 'year')) {
    return time.format('M-D H:m');
  }
  return time.format('YYYY-M-D H:m');
}
