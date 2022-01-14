import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export const authSlice = createSlice({
  name: 'auth',
  initialState: {
    value: window.localStorage.getItem('user_auth') as null | string,
  },
  reducers: {
    logout: (state) => {
      window.localStorage.removeItem('user_auth');
      state.value = null;
    },
    login: (state, action: PayloadAction<string>) => {
      window.localStorage.setItem('user_auth', action.payload);
      state.value = action.payload;
    },
  },
});

export const { logout, login } = authSlice.actions;

export default authSlice.reducer;
