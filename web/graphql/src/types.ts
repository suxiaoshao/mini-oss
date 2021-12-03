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
  /** Returns the sum of a and b */
  add: Scalars['Int'];
};

export type QueryRootAddArgs = {
  a: Scalars['Int'];
  b: Scalars['Int'];
};

export type AddQueryVariables = Exact<{ [key: string]: never }>;

export type AddQuery = { __typename?: 'QueryRoot'; add: number };

export const AddDocument = gql`
  query add {
    add(a: 1, b: 2)
  }
`;

/**
 * __useAddQuery__
 *
 * To run a query within a React component, call `useAddQuery` and pass it any options that fit your needs.
 * When your component renders, `useAddQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useAddQuery({
 *   variables: {
 *   },
 * });
 */
export function useAddQuery(baseOptions?: Apollo.QueryHookOptions<AddQuery, AddQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<AddQuery, AddQueryVariables>(AddDocument, options);
}
export function useAddLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<AddQuery, AddQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<AddQuery, AddQueryVariables>(AddDocument, options);
}
export type AddQueryHookResult = ReturnType<typeof useAddQuery>;
export type AddLazyQueryHookResult = ReturnType<typeof useAddLazyQuery>;
export type AddQueryResult = Apollo.QueryResult<AddQuery, AddQueryVariables>;
