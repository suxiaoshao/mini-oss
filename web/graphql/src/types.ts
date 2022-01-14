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

export type MutationRoot = {
  __typename?: 'MutationRoot';
  /** 用户创建 */
  manageUserCreate: UserInfo;
  /** 用户更新 */
  manageUserDelete: Scalars['Boolean'];
  /** 用户更新 */
  manageUserUpdate: UserInfo;
  /** 用户更新密码 */
  updateInfo: UserInfo;
  /** 用户更新密码 */
  updatePassword: Scalars['String'];
};

export type MutationRootManageUserCreateArgs = {
  auth: Scalars['String'];
  description?: InputMaybe<Scalars['String']>;
  name: Scalars['String'];
  password: Scalars['String'];
};

export type MutationRootManageUserDeleteArgs = {
  auth: Scalars['String'];
  name: Scalars['String'];
};

export type MutationRootManageUserUpdateArgs = {
  auth: Scalars['String'];
  description?: InputMaybe<Scalars['String']>;
  name: Scalars['String'];
};

export type MutationRootUpdateInfoArgs = {
  auth: Scalars['String'];
  description?: InputMaybe<Scalars['String']>;
};

export type MutationRootUpdatePasswordArgs = {
  auth: Scalars['String'];
  newPassword: Scalars['String'];
  oldPassword: Scalars['String'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
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

export type QueryRootManagerLoginArgs = {
  name: Scalars['String'];
  password: Scalars['String'];
};

export type QueryRootSelfUserInfoArgs = {
  auth: Scalars['String'];
};

export type QueryRootUserInfoArgs = {
  auth: Scalars['String'];
  name: Scalars['String'];
};

export type QueryRootUserListArgs = {
  auth: Scalars['String'];
  limit: Scalars['Int'];
  offset: Scalars['Int'];
};

export type QueryRootUserLoginArgs = {
  name: Scalars['String'];
  password: Scalars['String'];
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
  name: Scalars['String'];
  password: Scalars['String'];
}>;

export type UserLoginQuery = { __typename?: 'QueryRoot'; userLogin: string };

export type ManagerLoginQueryVariables = Exact<{
  name: Scalars['String'];
  password: Scalars['String'];
}>;

export type ManagerLoginQuery = { __typename?: 'QueryRoot'; managerLogin: string };

export type UserListQueryVariables = Exact<{
  limit: Scalars['Int'];
  offset: Scalars['Int'];
  auth: Scalars['String'];
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
  name: Scalars['String'];
  auth: Scalars['String'];
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
  name: Scalars['String'];
  password: Scalars['String'];
  auth: Scalars['String'];
  description?: InputMaybe<Scalars['String']>;
}>;

export type UserCreateMutation = {
  __typename?: 'MutationRoot';
  manageUserCreate: { __typename?: 'UserInfo'; name: string };
};

export type UserUpdateMutationVariables = Exact<{
  name: Scalars['String'];
  auth: Scalars['String'];
  description?: InputMaybe<Scalars['String']>;
}>;

export type UserUpdateMutation = {
  __typename?: 'MutationRoot';
  manageUserUpdate: { __typename?: 'UserInfo'; name: string };
};

export type UserDeleteMutationVariables = Exact<{
  name: Scalars['String'];
  auth: Scalars['String'];
}>;

export type UserDeleteMutation = { __typename?: 'MutationRoot'; manageUserDelete: boolean };

export type SelfUserInfoQueryVariables = Exact<{
  auth: Scalars['String'];
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
  auth: Scalars['String'];
  description?: InputMaybe<Scalars['String']>;
}>;

export type UpdateInfoMutation = { __typename?: 'MutationRoot'; updateInfo: { __typename?: 'UserInfo'; name: string } };

export type UpdatePasswordMutationVariables = Exact<{
  auth: Scalars['String'];
  newPassword: Scalars['String'];
  oldPassword: Scalars['String'];
}>;

export type UpdatePasswordMutation = { __typename?: 'MutationRoot'; updatePassword: string };

export const UserLoginDocument = gql`
  query userLogin($name: String!, $password: String!) {
    userLogin(name: $name, password: $password)
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
 *      name: // value for 'name'
 *      password: // value for 'password'
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
  query managerLogin($name: String!, $password: String!) {
    managerLogin(name: $name, password: $password)
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
 *      name: // value for 'name'
 *      password: // value for 'password'
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
  query userList($limit: Int!, $offset: Int!, $auth: String!) {
    userList(limit: $limit, offset: $offset, auth: $auth) {
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
 *      limit: // value for 'limit'
 *      offset: // value for 'offset'
 *      auth: // value for 'auth'
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
  query userInfo($name: String!, $auth: String!) {
    userInfo(name: $name, auth: $auth) {
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
 *      name: // value for 'name'
 *      auth: // value for 'auth'
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
  mutation userCreate($name: String!, $password: String!, $auth: String!, $description: String) {
    manageUserCreate(name: $name, password: $password, auth: $auth, description: $description) {
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
 *      name: // value for 'name'
 *      password: // value for 'password'
 *      auth: // value for 'auth'
 *      description: // value for 'description'
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
  mutation userUpdate($name: String!, $auth: String!, $description: String) {
    manageUserUpdate(name: $name, auth: $auth, description: $description) {
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
 *      name: // value for 'name'
 *      auth: // value for 'auth'
 *      description: // value for 'description'
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
  mutation userDelete($name: String!, $auth: String!) {
    manageUserDelete(name: $name, auth: $auth)
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
 *      name: // value for 'name'
 *      auth: // value for 'auth'
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
  query selfUserInfo($auth: String!) {
    selfUserInfo(auth: $auth) {
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
 *      auth: // value for 'auth'
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
  mutation updateInfo($auth: String!, $description: String) {
    updateInfo(auth: $auth, description: $description) {
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
 *      auth: // value for 'auth'
 *      description: // value for 'description'
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
  mutation updatePassword($auth: String!, $newPassword: String!, $oldPassword: String!) {
    updatePassword(auth: $auth, newPassword: $newPassword, oldPassword: $oldPassword)
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
 *      auth: // value for 'auth'
 *      newPassword: // value for 'newPassword'
 *      oldPassword: // value for 'oldPassword'
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
