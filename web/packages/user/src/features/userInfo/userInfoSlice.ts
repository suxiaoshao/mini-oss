import { createAsyncThunk, createSlice } from '@reduxjs/toolkit';
import { client } from 'common';
import { SelfUserInfoDocument, SelfUserInfoQuery, SelfUserInfoQueryVariables, UserInfo as GqlUserInfo } from 'graphql';

export type UserInfo = Omit<GqlUserInfo, '__typename'>;

export const updateUserInfo = createAsyncThunk('updateUserInfo', async (auth: string) => {
  const { data } = await client.query<SelfUserInfoQuery, SelfUserInfoQueryVariables>({
    query: SelfUserInfoDocument,
    variables: { data: { auth } },
  });
  return await data.selfUserInfo;
});

export const userInfoSlice = createSlice({
  name: 'userInfo',
  initialState: {
    name: '',
    createTime: 0,
    updateTime: 0,
    description: null,
  } as UserInfo,
  reducers: {},
  extraReducers: (builder) => {
    builder.addCase(updateUserInfo.fulfilled, (state, { payload }) => {
      state.name = payload.name;
      state.description = payload.description;
      state.createTime = payload.createTime;
      state.updateTime = payload.updateTime;
    });
  },
});

export default userInfoSlice.reducer;