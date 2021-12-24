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
  delete: Scalars['Int'];
};

export type MutationRootDeleteArgs = {
  a: Scalars['Int'];
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

export type MyAddQueryVariables = Exact<{ [key: string]: never }>;

export type MyAddQuery = { __typename?: 'QueryRoot'; userLogin: string };

export const MyAddDocument = gql`
  query myAdd {
    userLogin(name: "sushao", password: "sushao")
  }
`;

/**
 * __useMyAddQuery__
 *
 * To run a query within a React component, call `useMyAddQuery` and pass it any options that fit your needs.
 * When your component renders, `useMyAddQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useMyAddQuery({
 *   variables: {
 *   },
 * });
 */
export function useMyAddQuery(baseOptions?: Apollo.QueryHookOptions<MyAddQuery, MyAddQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<MyAddQuery, MyAddQueryVariables>(MyAddDocument, options);
}
export function useMyAddLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<MyAddQuery, MyAddQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<MyAddQuery, MyAddQueryVariables>(MyAddDocument, options);
}
export type MyAddQueryHookResult = ReturnType<typeof useMyAddQuery>;
export type MyAddLazyQueryHookResult = ReturnType<typeof useMyAddLazyQuery>;
export type MyAddQueryResult = Apollo.QueryResult<MyAddQuery, MyAddQueryVariables>;
