import { Box } from '@mui/material';
import { CustomEchart } from 'common';
import { EChartsOption } from 'echarts';
import { BucketInfoQuery } from 'graphql';
import prettyBytes from 'pretty-bytes';

export type ChartsProps = Pick<
  BucketInfoQuery['bucketInfo'],
  'objectCountChart' | 'objectSizeChart' | 'requestCountChart' | 'uploadSizeChart' | 'downloadSizeChart'
>;

export default function Charts({
  objectCountChart,
  objectSizeChart,
  requestCountChart,
  uploadSizeChart,
  downloadSizeChart,
}: ChartsProps): JSX.Element {
  const option: EChartsOption = {
    tooltip: {
      trigger: 'axis',
    },
    xAxis: {
      type: 'time',
      splitLine: {
        show: false,
      },
    },
    yAxis: {
      type: 'value',
      boundaryGap: [0, '20%'],
      splitLine: {
        show: false,
      },
    },
  };
  return (
    <Box>
      <CustomEchart
        option={{
          title: {
            text: '对象数量',
          },
          series: [
            {
              type: 'line',
              smooth: true,
              data: objectCountChart.map(({ value, time }) => {
                return [time, value];
              }),
            },
          ],
          ...option,
        }}
        sx={{ marginTop: (theme) => theme.spacing(2) }}
      />
      <CustomEchart
        option={{
          ...option,
          title: {
            text: '对象大小',
          },
          tooltip: {
            trigger: 'axis',
            valueFormatter: (value) => prettyBytes(Number(value)),
          },
          yAxis: {
            type: 'value',
            boundaryGap: [0, '20%'],
            splitLine: {
              show: false,
            },
            axisLabel: {
              formatter: (value: number) => prettyBytes(value),
            },
          },
          series: [
            {
              type: 'line',
              smooth: true,
              data: objectSizeChart.map(({ value, time }) => {
                return [time, value];
              }),
            },
          ],
        }}
        sx={{ marginTop: (theme) => theme.spacing(2) }}
      />
      <CustomEchart
        option={{
          title: {
            text: '请求数量',
          },
          series: [
            {
              type: 'line',
              smooth: true,
              data: requestCountChart.map(({ value, startTime, endTime }) => {
                return [(startTime + endTime) / 2, value];
              }),
            },
          ],
          ...option,
        }}
        sx={{ marginTop: (theme) => theme.spacing(2) }}
      />
      <CustomEchart
        option={{
          ...option,
          title: {
            text: '流量',
          },
          legend: {
            data: ['上传流量', '下载流量'],
          },
          yAxis: {
            type: 'value',
            boundaryGap: [0, '20%'],
            splitLine: {
              show: false,
            },
            axisLabel: {
              formatter: (value: number) => prettyBytes(value),
            },
          },
          tooltip: {
            trigger: 'axis',
            valueFormatter: (value) => prettyBytes(Number(value)),
          },
          series: [
            {
              type: 'line',
              smooth: true,
              data: uploadSizeChart.map(({ value, startTime, endTime }) => {
                return [(startTime + endTime) / 2, value];
              }),
              name: '上传流量',
            },
            {
              type: 'line',
              smooth: true,
              data: downloadSizeChart.map(({ value, startTime, endTime }) => {
                return [(startTime + endTime) / 2, value];
              }),
              name: '下载流量',
            },
          ],
        }}
        sx={{ marginTop: (theme) => theme.spacing(2) }}
      />
    </Box>
  );
}
