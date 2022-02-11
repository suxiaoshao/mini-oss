import { store } from '@/app/store';
import axios, { AxiosError } from 'axios';
import { enqueueSnackbar } from 'common';

const instance = axios.create({
  baseURL: 'http://as-sushao.open.mini-oss.top',
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
