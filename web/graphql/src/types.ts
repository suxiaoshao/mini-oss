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

export type QueryRoot = {
  __typename?: 'QueryRoot';
  /** 管理员登陆 */
  managerLogin: Scalars['String'];
  /** 用户登陆 */
  userLogin: Scalars['String'];
};

export type QueryRootManagerLoginArgs = {
  name: Scalars['String'];
  password: Scalars['String'];
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
