import { gql } from '@apollo/client';
import * as Apollo from '@apollo/client';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
const defaultOptions = {};
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

/** 访问权限类型 */
export enum Access {
  /** 开放 */
  Open = 'OPEN',
  /** 不开放 */
  Private = 'PRIVATE',
  /** 读开放 */
  ReadOpen = 'READ_OPEN',
}

export type BucketInfo = {
  __typename?: 'BucketInfo';
  /** 访问权限 */
  access: Access;
  /** 创建时间 */
  createTime: Scalars['Int'];
  /** 名字 */
  name: Scalars['String'];
  /** 更新时间 */
  updateTime: Scalars['Int'];
  /** 用户名 */
  userName: Scalars['String'];
};

export type BucketList = {
  __typename?: 'BucketList';
  /** 数据 */
  data: Array<BucketInfo>;
  /** 总数 */
  total: Scalars['Int'];
};

export type CreateBucketRequest = {
  /** 访问控制 */
  access: Scalars['Int'];
  /** 用户凭证 */
  auth: Scalars['String'];
  /** 名字 */
  name: Scalars['String'];
};

export type CreateUserRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
  /** 描述 */
  description?: InputMaybe<Scalars['String']>;
  /** 账号 */
  name: Scalars['String'];
  /** 密码 */
  password: Scalars['String'];
};

export type DeleteBucketRequest = {
  /** 用户凭证 */
  auth: Scalars['String'];
  /** 名字 */
  name: Scalars['String'];
};

export type DeleteUserRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
  /** 账号 */
  name: Scalars['String'];
};

export type GetListRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
  /** 获取多少数据 */
  limit: Scalars['Int'];
  /** 偏移量 */
  offset: Scalars['Int'];
};

export type GetUserInfoRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
};

export type GetUserRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
  /** 用户名 */
  name: Scalars['String'];
};

export type LoginRequest = {
  /** 账号 */
  name: Scalars['String'];
  /** 密码 */
  password: Scalars['String'];
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  /** 创建存储桶 */
  createBucket: BucketInfo;
  /** 删除存储桶 */
  deleteBucket: Scalars['Boolean'];
  /** 用户创建 */
  manageUserCreate: UserInfo;
  /** 用户删除 */
  manageUserDelete: Scalars['Boolean'];
  /** 用户更新 */
  manageUserUpdate: UserInfo;
  /** 更新存储桶 */
  updateBucket: BucketInfo;
  /** 用户更新信息 */
  updateInfo: UserInfo;
  /** 用户更新密码 */
  updatePassword: Scalars['String'];
};

export type MutationRootCreateBucketArgs = {
  data: CreateBucketRequest;
};

export type MutationRootDeleteBucketArgs = {
  data: DeleteBucketRequest;
};

export type MutationRootManageUserCreateArgs = {
  data: CreateUserRequest;
};

export type MutationRootManageUserDeleteArgs = {
  data: DeleteUserRequest;
};

export type MutationRootManageUserUpdateArgs = {
  data: UpdateUserRequest;
};

export type MutationRootUpdateBucketArgs = {
  data: UpdateBucketRequest;
};

export type MutationRootUpdateInfoArgs = {
  data: UpdateUserInfoRequest;
};

export type MutationRootUpdatePasswordArgs = {
  data: UpdatePasswordRequest;
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  /** 用户存储桶列表 */
  bucketList: BucketList;
  /** 管理员登陆 */
  managerLogin: Scalars['String'];
  /** 获取自身用户信息 */
  selfUserInfo: UserInfo;
  /** 用户信息 */
  userInfo: UserInfo;
  /** 用户列表 */
  userList: UserList;
  /** 用户登陆 */
  userLogin: Scalars['String'];
};

export type QueryRootBucketListArgs = {
  data: GetListRequest;
};

export type QueryRootManagerLoginArgs = {
  data: LoginRequest;
};

export type QueryRootSelfUserInfoArgs = {
  data: GetUserInfoRequest;
};

export type QueryRootUserInfoArgs = {
  data: GetUserRequest;
};

export type QueryRootUserListArgs = {
  data: GetListRequest;
};

export type QueryRootUserLoginArgs = {
  data: LoginRequest;
};

export type UpdateBucketRequest = {
  /** 访问控制 */
  access: Scalars['Int'];
  /** 用户凭证 */
  auth: Scalars['String'];
  /** 名字 */
  name: Scalars['String'];
};

export type UpdatePasswordRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
  /** 新密码 */
  newPassword: Scalars['String'];
  /** 旧密码 */
  oldPassword: Scalars['String'];
};

export type UpdateUserInfoRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
  /** 描述 */
  description?: InputMaybe<Scalars['String']>;
};

export type UpdateUserRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
  /** 描述 */
  description?: InputMaybe<Scalars['String']>;
  /** 账号 */
  name: Scalars['String'];
};

export type UserInfo = {
  __typename?: 'UserInfo';
  /** 创建时间 */
  createTime: Scalars['Int'];
  /** 描述 */
  description?: Maybe<Scalars['String']>;
  /** 名字 */
  name: Scalars['String'];
  /** 更新时间 */
  updateTime: Scalars['Int'];
};

export type UserList = {
  __typename?: 'UserList';
  /** 数据 */
  data: Array<UserInfo>;
  /** 总数 */
  total: Scalars['Int'];
};

export type UserLoginQueryVariables = Exact<{
  data: LoginRequest;
}>;

export type UserLoginQuery = { __typename?: 'QueryRoot'; userLogin: string };

export type ManagerLoginQueryVariables = Exact<{
  data: LoginRequest;
}>;

export type ManagerLoginQuery = { __typename?: 'QueryRoot'; managerLogin: string };

export type UserListQueryVariables = Exact<{
  data: GetListRequest;
}>;

export type UserListQuery = {
  __typename?: 'QueryRoot';
  userList: {
    __typename?: 'UserList';
    total: number;
    data: Array<{
      __typename?: 'UserInfo';
      name: string;
      description?: string | null | undefined;
      createTime: number;
      updateTime: number;
    }>;
  };
};

export type UserInfoQueryVariables = Exact<{
  data: GetUserRequest;
}>;

export type UserInfoQuery = {
  __typename?: 'QueryRoot';
  userInfo: {
    __typename?: 'UserInfo';
    name: string;
    description?: string | null | undefined;
    createTime: number;
    updateTime: number;
  };
};

export type UserCreateMutationVariables = Exact<{
  data: CreateUserRequest;
}>;

export type UserCreateMutation = {
  __typename?: 'MutationRoot';
  manageUserCreate: { __typename?: 'UserInfo'; name: string };
};

export type UserUpdateMutationVariables = Exact<{
  data: UpdateUserRequest;
}>;

export type UserUpdateMutation = {
  __typename?: 'MutationRoot';
  manageUserUpdate: { __typename?: 'UserInfo'; name: string };
};

export type UserDeleteMutationVariables = Exact<{
  data: DeleteUserRequest;
}>;

export type UserDeleteMutation = { __typename?: 'MutationRoot'; manageUserDelete: boolean };

export type SelfUserInfoQueryVariables = Exact<{
  data: GetUserInfoRequest;
}>;

export type SelfUserInfoQuery = {
  __typename?: 'QueryRoot';
  selfUserInfo: {
    __typename?: 'UserInfo';
    name: string;
    createTime: number;
    updateTime: number;
    description?: string | null | undefined;
  };
};

export type UpdateInfoMutationVariables = Exact<{
  data: UpdateUserInfoRequest;
}>;

export type UpdateInfoMutation = { __typename?: 'MutationRoot'; updateInfo: { __typename?: 'UserInfo'; name: string } };

export type UpdatePasswordMutationVariables = Exact<{
  data: UpdatePasswordRequest;
}>;

export type UpdatePasswordMutation = { __typename?: 'MutationRoot'; updatePassword: string };

export const UserLoginDocument = gql`
  query userLogin($data: LoginRequest!) {
    userLogin(data: $data)
  }
`;

/**
 * __useUserLoginQuery__
 *
 * To run a query within a React component, call `useUserLoginQuery` and pass it any options that fit your needs.
 * When your component renders, `useUserLoginQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useUserLoginQuery({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUserLoginQuery(baseOptions: Apollo.QueryHookOptions<UserLoginQuery, UserLoginQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<UserLoginQuery, UserLoginQueryVariables>(UserLoginDocument, options);
}
export function useUserLoginLazyQuery(
  baseOptions?: Apollo.LazyQueryHookOptions<UserLoginQuery, UserLoginQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<UserLoginQuery, UserLoginQueryVariables>(UserLoginDocument, options);
}
export type UserLoginQueryHookResult = ReturnType<typeof useUserLoginQuery>;
export type UserLoginLazyQueryHookResult = ReturnType<typeof useUserLoginLazyQuery>;
export type UserLoginQueryResult = Apollo.QueryResult<UserLoginQuery, UserLoginQueryVariables>;
export const ManagerLoginDocument = gql`
  query managerLogin($data: LoginRequest!) {
    managerLogin(data: $data)
  }
`;

/**
 * __useManagerLoginQuery__
 *
 * To run a query within a React component, call `useManagerLoginQuery` and pass it any options that fit your needs.
 * When your component renders, `useManagerLoginQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useManagerLoginQuery({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useManagerLoginQuery(
  baseOptions: Apollo.QueryHookOptions<ManagerLoginQuery, ManagerLoginQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<ManagerLoginQuery, ManagerLoginQueryVariables>(ManagerLoginDocument, options);
}
export function useManagerLoginLazyQuery(
  baseOptions?: Apollo.LazyQueryHookOptions<ManagerLoginQuery, ManagerLoginQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<ManagerLoginQuery, ManagerLoginQueryVariables>(ManagerLoginDocument, options);
}
export type ManagerLoginQueryHookResult = ReturnType<typeof useManagerLoginQuery>;
export type ManagerLoginLazyQueryHookResult = ReturnType<typeof useManagerLoginLazyQuery>;
export type ManagerLoginQueryResult = Apollo.QueryResult<ManagerLoginQuery, ManagerLoginQueryVariables>;
export const UserListDocument = gql`
  query userList($data: GetListRequest!) {
    userList(data: $data) {
      data {
        name
        description
        createTime
        updateTime
      }
      total
    }
  }
`;

/**
 * __useUserListQuery__
 *
 * To run a query within a React component, call `useUserListQuery` and pass it any options that fit your needs.
 * When your component renders, `useUserListQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useUserListQuery({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUserListQuery(baseOptions: Apollo.QueryHookOptions<UserListQuery, UserListQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<UserListQuery, UserListQueryVariables>(UserListDocument, options);
}
export function useUserListLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<UserListQuery, UserListQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<UserListQuery, UserListQueryVariables>(UserListDocument, options);
}
export type UserListQueryHookResult = ReturnType<typeof useUserListQuery>;
export type UserListLazyQueryHookResult = ReturnType<typeof useUserListLazyQuery>;
export type UserListQueryResult = Apollo.QueryResult<UserListQuery, UserListQueryVariables>;
export const UserInfoDocument = gql`
  query userInfo($data: GetUserRequest!) {
    userInfo(data: $data) {
      name
      description
      createTime
      updateTime
    }
  }
`;

/**
 * __useUserInfoQuery__
 *
 * To run a query within a React component, call `useUserInfoQuery` and pass it any options that fit your needs.
 * When your component renders, `useUserInfoQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useUserInfoQuery({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUserInfoQuery(baseOptions: Apollo.QueryHookOptions<UserInfoQuery, UserInfoQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<UserInfoQuery, UserInfoQueryVariables>(UserInfoDocument, options);
}
export function useUserInfoLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<UserInfoQuery, UserInfoQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<UserInfoQuery, UserInfoQueryVariables>(UserInfoDocument, options);
}
export type UserInfoQueryHookResult = ReturnType<typeof useUserInfoQuery>;
export type UserInfoLazyQueryHookResult = ReturnType<typeof useUserInfoLazyQuery>;
export type UserInfoQueryResult = Apollo.QueryResult<UserInfoQuery, UserInfoQueryVariables>;
export const UserCreateDocument = gql`
  mutation userCreate($data: CreateUserRequest!) {
    manageUserCreate(data: $data) {
      name
    }
  }
`;
export type UserCreateMutationFn = Apollo.MutationFunction<UserCreateMutation, UserCreateMutationVariables>;

/**
 * __useUserCreateMutation__
 *
 * To run a mutation, you first call `useUserCreateMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useUserCreateMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [userCreateMutation, { data, loading, error }] = useUserCreateMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUserCreateMutation(
  baseOptions?: Apollo.MutationHookOptions<UserCreateMutation, UserCreateMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<UserCreateMutation, UserCreateMutationVariables>(UserCreateDocument, options);
}
export type UserCreateMutationHookResult = ReturnType<typeof useUserCreateMutation>;
export type UserCreateMutationResult = Apollo.MutationResult<UserCreateMutation>;
export type UserCreateMutationOptions = Apollo.BaseMutationOptions<UserCreateMutation, UserCreateMutationVariables>;
export const UserUpdateDocument = gql`
  mutation userUpdate($data: UpdateUserRequest!) {
    manageUserUpdate(data: $data) {
      name
    }
  }
`;
export type UserUpdateMutationFn = Apollo.MutationFunction<UserUpdateMutation, UserUpdateMutationVariables>;

/**
 * __useUserUpdateMutation__
 *
 * To run a mutation, you first call `useUserUpdateMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useUserUpdateMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [userUpdateMutation, { data, loading, error }] = useUserUpdateMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUserUpdateMutation(
  baseOptions?: Apollo.MutationHookOptions<UserUpdateMutation, UserUpdateMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<UserUpdateMutation, UserUpdateMutationVariables>(UserUpdateDocument, options);
}
export type UserUpdateMutationHookResult = ReturnType<typeof useUserUpdateMutation>;
export type UserUpdateMutationResult = Apollo.MutationResult<UserUpdateMutation>;
export type UserUpdateMutationOptions = Apollo.BaseMutationOptions<UserUpdateMutation, UserUpdateMutationVariables>;
export const UserDeleteDocument = gql`
  mutation userDelete($data: DeleteUserRequest!) {
    manageUserDelete(data: $data)
  }
`;
export type UserDeleteMutationFn = Apollo.MutationFunction<UserDeleteMutation, UserDeleteMutationVariables>;

/**
 * __useUserDeleteMutation__
 *
 * To run a mutation, you first call `useUserDeleteMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useUserDeleteMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [userDeleteMutation, { data, loading, error }] = useUserDeleteMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUserDeleteMutation(
  baseOptions?: Apollo.MutationHookOptions<UserDeleteMutation, UserDeleteMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<UserDeleteMutation, UserDeleteMutationVariables>(UserDeleteDocument, options);
}
export type UserDeleteMutationHookResult = ReturnType<typeof useUserDeleteMutation>;
export type UserDeleteMutationResult = Apollo.MutationResult<UserDeleteMutation>;
export type UserDeleteMutationOptions = Apollo.BaseMutationOptions<UserDeleteMutation, UserDeleteMutationVariables>;
export const SelfUserInfoDocument = gql`
  query selfUserInfo($data: GetUserInfoRequest!) {
    selfUserInfo(data: $data) {
      name
      createTime
      updateTime
      description
    }
  }
`;

/**
 * __useSelfUserInfoQuery__
 *
 * To run a query within a React component, call `useSelfUserInfoQuery` and pass it any options that fit your needs.
 * When your component renders, `useSelfUserInfoQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useSelfUserInfoQuery({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useSelfUserInfoQuery(
  baseOptions: Apollo.QueryHookOptions<SelfUserInfoQuery, SelfUserInfoQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<SelfUserInfoQuery, SelfUserInfoQueryVariables>(SelfUserInfoDocument, options);
}
export function useSelfUserInfoLazyQuery(
  baseOptions?: Apollo.LazyQueryHookOptions<SelfUserInfoQuery, SelfUserInfoQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<SelfUserInfoQuery, SelfUserInfoQueryVariables>(SelfUserInfoDocument, options);
}
export type SelfUserInfoQueryHookResult = ReturnType<typeof useSelfUserInfoQuery>;
export type SelfUserInfoLazyQueryHookResult = ReturnType<typeof useSelfUserInfoLazyQuery>;
export type SelfUserInfoQueryResult = Apollo.QueryResult<SelfUserInfoQuery, SelfUserInfoQueryVariables>;
export const UpdateInfoDocument = gql`
  mutation updateInfo($data: UpdateUserInfoRequest!) {
    updateInfo(data: $data) {
      name
    }
  }
`;
export type UpdateInfoMutationFn = Apollo.MutationFunction<UpdateInfoMutation, UpdateInfoMutationVariables>;

/**
 * __useUpdateInfoMutation__
 *
 * To run a mutation, you first call `useUpdateInfoMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useUpdateInfoMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [updateInfoMutation, { data, loading, error }] = useUpdateInfoMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUpdateInfoMutation(
  baseOptions?: Apollo.MutationHookOptions<UpdateInfoMutation, UpdateInfoMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<UpdateInfoMutation, UpdateInfoMutationVariables>(UpdateInfoDocument, options);
}
export type UpdateInfoMutationHookResult = ReturnType<typeof useUpdateInfoMutation>;
export type UpdateInfoMutationResult = Apollo.MutationResult<UpdateInfoMutation>;
export type UpdateInfoMutationOptions = Apollo.BaseMutationOptions<UpdateInfoMutation, UpdateInfoMutationVariables>;
export const UpdatePasswordDocument = gql`
  mutation updatePassword($data: UpdatePasswordRequest!) {
    updatePassword(data: $data)
  }
`;
export type UpdatePasswordMutationFn = Apollo.MutationFunction<UpdatePasswordMutation, UpdatePasswordMutationVariables>;

/**
 * __useUpdatePasswordMutation__
 *
 * To run a mutation, you first call `useUpdatePasswordMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useUpdatePasswordMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [updatePasswordMutation, { data, loading, error }] = useUpdatePasswordMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUpdatePasswordMutation(
  baseOptions?: Apollo.MutationHookOptions<UpdatePasswordMutation, UpdatePasswordMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<UpdatePasswordMutation, UpdatePasswordMutationVariables>(UpdatePasswordDocument, options);
}
export type UpdatePasswordMutationHookResult = ReturnType<typeof useUpdatePasswordMutation>;
export type UpdatePasswordMutationResult = Apollo.MutationResult<UpdatePasswordMutation>;
export type UpdatePasswordMutationOptions = Apollo.BaseMutationOptions<
  UpdatePasswordMutation,
  UpdatePasswordMutationVariables
>;
