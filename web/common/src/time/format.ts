import { dayjs } from './index';

/** 格式化时间 */
export function format(timestamp?: number): string {
  if (timestamp == null || timestamp == undefined) {
    return '~';
  }
  const time = dayjs(timestamp);
  const now = dayjs();
  if (now.isSame(time, 'day')) {
    return time.format('HH:mm');
  }
  if (now.isSame(time, 'year')) {
    return time.format('M月D日 HH:mm');
  }
  return time.format('YYYY年M月D日 HH:mm');
}
