import { client } from '@/utils/graphql';
import { createAsyncThunk, createSlice } from '@reduxjs/toolkit';
import { SelfUserInfoDocument, SelfUserInfoQuery, SelfUserInfoQueryVariables, UserInfo as GqlUserInfo } from 'graphql';

export type UserInfo = Omit<GqlUserInfo, '__typename'>;

export const updateUserInfo = createAsyncThunk('updateUserInfo', async () => {
  const { data } = await client.query<SelfUserInfoQuery, SelfUserInfoQueryVariables>({
    query: SelfUserInfoDocument,
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
