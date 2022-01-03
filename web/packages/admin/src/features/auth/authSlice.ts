import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export const authSlice = createSlice({
  name: 'auth',
  initialState: {
    value: null as null | string,
  },
  reducers: {
    logout: (state) => {
      // Redux Toolkit allows us to write "mutating" logic in reducers. It
      // doesn't actually mutate the state because it uses the immer library,
      // which detects changes to a "draft state" and produces a brand new
      // immutable state based off those changes
      state.value = null;
    },
    login: (state, action: PayloadAction<string>) => {
      state.value = action.payload;
    },
  },
});

export const { logout, login } = authSlice.actions;

export default authSlice.reducer;
