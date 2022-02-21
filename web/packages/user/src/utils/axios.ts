import { store } from '@/app/store';
import axios, { AxiosError } from 'axios';
import { enqueueSnackbar } from 'common';

export function getPath(bucketName: string, path: string, filename: string): string {
  const url = import.meta.env.VITE_OPEN_URL ?? 'open.mini-oss.sushao.top';
  return `http://${bucketName}.${url}${path}${encodeURIComponent(filename)}`;
}

const instance = axios.create({
  withCredentials: true,
});
instance.interceptors.request.use(
  (config) => {
    const auth = store.getState().auth.value;
    if (auth) {
      if (config.headers) {
        config.headers.Authorization = auth;
      } else {
        config.headers = { Authorization: auth };
      }
    }
    return config;
  },
  (error) => {
    // Do something with request error
    return Promise.reject(error);
  },
);

instance.interceptors.response.use(
  (response) => {
    if (response.data.message) {
      console.log(response.data);
      enqueueSnackbar(response.data.message);
      throw new Error(response.data.message);
    }
    return response.data.data;
  },
  (responseError: AxiosError) => {
    if (responseError.response) {
      console.log(responseError.response.data);
      enqueueSnackbar(responseError.response.data.message);
    } else {
      enqueueSnackbar(`网络错误:${responseError.message}`);
    }
    throw responseError;
  },
);

export const axiosInstance = instance;
