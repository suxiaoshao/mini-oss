import { gql } from '@apollo/client';
import * as Apollo from '@apollo/client';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
const defaultOptions = {} as const;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

/** 访问权限类型 */
export enum BucketAccess {
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
  access: BucketAccess;
  /** 创建时间 */
  createTime: Scalars['Int'];
  /** 名字 */
  name: Scalars['String'];
  /** 更新时间 */
  updateTime: Scalars['Int'];
  /** 用户名 */
  username: Scalars['String'];
};

export type BucketList = {
  __typename?: 'BucketList';
  /** 数据 */
  data: Array<BucketInfo>;
  /** 总数 */
  total: Scalars['Int'];
};

export type CreateBucketRequest = {
  access: BucketAccess;
  auth: Scalars['String'];
  name: Scalars['String'];
};

export type CreateFolderRequest = {
  /** 访问控制 */
  access: FolderAccess;
  /** 用户凭证 */
  auth?: InputMaybe<Scalars['String']>;
  /** bucket 名 */
  bucketName: Scalars['String'];
  /** 路径 */
  fatherPath: Scalars['String'];
  /** 路径 */
  path: Scalars['String'];
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

export type DeleteFolderRequest = {
  /** 用户凭证 */
  auth?: InputMaybe<Scalars['String']>;
  /** bucket 名 */
  bucketName: Scalars['String'];
  /** 名字 */
  path: Scalars['String'];
};

export type DeleteObjectRequest = {
  /** 访问控制 */
  auth?: InputMaybe<Scalars['String']>;
  /** bucket 名 */
  bucketName: Scalars['String'];
  /** 文件名 */
  filename: Scalars['String'];
  /** 路径 */
  path: Scalars['String'];
};

export type DeleteUserRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
  /** 账号 */
  name: Scalars['String'];
};

/** 访问权限类型 */
export enum FolderAccess {
  /** 继承 */
  InheritanceFolder = 'INHERITANCE_FOLDER',
  /** 开放 */
  OpenFolder = 'OPEN_FOLDER',
  /** 不开放 */
  PrivateFolder = 'PRIVATE_FOLDER',
  /** 读开放 */
  ReadOpenFolder = 'READ_OPEN_FOLDER',
}

export type FolderInfo = {
  __typename?: 'FolderInfo';
  /** 访问控制 */
  access: FolderAccess;
  /** bucket 名 */
  bucketName: Scalars['String'];
  /** 创建时间 */
  createTime: Scalars['Int'];
  /** 路径 */
  fatherPath: Scalars['String'];
  folderCount: Scalars['Int'];
  folderName: Scalars['String'];
  objectCount: Scalars['Int'];
  objectSize: Scalars['Int'];
  /** 路径 */
  path: Scalars['String'];
  /** 创建时间 */
  updateTime: Scalars['Int'];
};

export type FolderItem = FolderInfo | ObjectInfo;

export type FolderList = {
  __typename?: 'FolderList';
  data: Array<FolderItem>;
  total: Scalars['Int'];
};

export type GetBucketRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
  /** bucket 名 */
  bucketName: Scalars['String'];
};

export type GetFolderListRequest = {
  /** 身份验证 */
  auth?: InputMaybe<Scalars['String']>;
  /** bucket 名 */
  bucketName: Scalars['String'];
  /** 获取多少数据 */
  limit: Scalars['Int'];
  /** 偏移量 */
  offset: Scalars['Int'];
  /** 路径 */
  path: Scalars['String'];
};

export type GetFolderRequest = {
  /** 身份验证 */
  auth?: InputMaybe<Scalars['String']>;
  /** bucket 名 */
  bucketName: Scalars['String'];
  /** 路径 */
  path: Scalars['String'];
};

export type GetListRequest = {
  /** 身份验证 */
  auth: Scalars['String'];
  /** 获取多少数据 */
  limit: Scalars['Int'];
  /** 偏移量 */
  offset: Scalars['Int'];
};

export type GetObjectRequest = {
  /** 身份验证 */
  auth?: InputMaybe<Scalars['String']>;
  /** bucket 名 */
  bucketName: Scalars['String'];
  /** 文件名 */
  filename: Scalars['String'];
  /** 路径 */
  path: Scalars['String'];
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

export type Header = {
  __typename?: 'Header';
  key: Scalars['String'];
  value: Scalars['String'];
};

export type HeaderType = {
  key: Scalars['String'];
  value: Scalars['String'];
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
  /** 创建目录 */
  createFolder: FolderInfo;
  /** 删除存储桶 */
  deleteBucket: Scalars['Boolean'];
  /** 删除目录 */
  deleteFolder: Scalars['Boolean'];
  /** 删除对象 */
  deleteObject: Scalars['Boolean'];
  /** 用户创建 */
  manageUserCreate: UserInfo;
  /** 用户删除 */
  manageUserDelete: Scalars['Boolean'];
  /** 用户更新 */
  manageUserUpdate: UserInfo;
  /** 更新存储桶 */
  updateBucket: BucketInfo;
  /** 更新目录 */
  updateFolder: FolderInfo;
  /** 用户更新信息 */
  updateInfo: UserInfo;
  /** 更新对象 */
  updateObject: ObjectInfo;
  /** 用户更新密码 */
  updatePassword: Scalars['String'];
};

export type MutationRootCreateBucketArgs = {
  data: CreateBucketRequest;
};

export type MutationRootCreateFolderArgs = {
  data: CreateFolderRequest;
};

export type MutationRootDeleteBucketArgs = {
  data: DeleteBucketRequest;
};

export type MutationRootDeleteFolderArgs = {
  data: DeleteFolderRequest;
};

export type MutationRootDeleteObjectArgs = {
  data: DeleteObjectRequest;
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

export type MutationRootUpdateFolderArgs = {
  data: UpdateFolderRequest;
};

export type MutationRootUpdateInfoArgs = {
  data: UpdateUserInfoRequest;
};

export type MutationRootUpdateObjectArgs = {
  data: UpdateObjectRequest;
};

export type MutationRootUpdatePasswordArgs = {
  data: UpdatePasswordRequest;
};

export enum ObjectAccess {
  /** 继承 */
  InheritanceObject = 'INHERITANCE_OBJECT',
  /** 不开放 */
  PrivateObject = 'PRIVATE_OBJECT',
  /** 读开放 */
  ReadOpenObject = 'READ_OPEN_OBJECT',
}

export type ObjectInfo = {
  __typename?: 'ObjectInfo';
  /** 访问控制 */
  access: ObjectAccess;
  /** 摘要 */
  blake3: Scalars['String'];
  /** bucket 名 */
  bucketName: Scalars['String'];
  /** 创建时间 */
  createTime: Scalars['Int'];
  /** 文件名 */
  filename: Scalars['String'];
  /** 自定义 header */
  headers: Array<Header>;
  /** 路径 */
  path: Scalars['String'];
  /** 大小 */
  size: Scalars['Int'];
  /** 创建时间 */
  updateTime: Scalars['Int'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  /** 获取存储桶信息 */
  bucketInfo: BucketInfo;
  /** 用户存储桶列表 */
  bucketList: BucketList;
  /** 获取文件夹信息 */
  folderInfo: FolderInfo;
  /** 文件夹列表 */
  folderList: FolderList;
  /** 管理员登陆 */
  managerLogin: Scalars['String'];
  /** 获取对象信息 */
  objectInfo: ObjectInfo;
  /** 获取自身用户信息 */
  selfUserInfo: UserInfo;
  /** 用户信息 */
  userInfo: UserInfo;
  /** 用户列表 */
  userList: UserList;
  /** 用户登陆 */
  userLogin: Scalars['String'];
};

export type QueryRootBucketInfoArgs = {
  data: GetBucketRequest;
};

export type QueryRootBucketListArgs = {
  data: GetListRequest;
};

export type QueryRootFolderInfoArgs = {
  data: GetFolderRequest;
};

export type QueryRootFolderListArgs = {
  data: GetFolderListRequest;
};

export type QueryRootManagerLoginArgs = {
  data: LoginRequest;
};

export type QueryRootObjectInfoArgs = {
  data: GetObjectRequest;
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
  access: BucketAccess;
  auth: Scalars['String'];
  name: Scalars['String'];
};

export type UpdateFolderRequest = {
  /** 访问控制 */
  access: FolderAccess;
  /** 用户凭证 */
  auth?: InputMaybe<Scalars['String']>;
  /** bucket 名 */
  bucketName: Scalars['String'];
  /** 路径 */
  path: Scalars['String'];
};

export type UpdateObjectRequest = {
  /** 访问控制 */
  access: ObjectAccess;
  /** 访问控制 */
  auth?: InputMaybe<Scalars['String']>;
  /** bucket 名 */
  bucketName: Scalars['String'];
  /** 旧文件名 */
  filename: Scalars['String'];
  /** 自定义 header */
  headers: Array<HeaderType>;
  /** 新文件名 */
  newFilename: Scalars['String'];
  /** 路径 */
  path: Scalars['String'];
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

export type BucketListQueryVariables = Exact<{
  data: GetListRequest;
}>;

export type BucketListQuery = {
  __typename?: 'QueryRoot';
  bucketList: {
    __typename?: 'BucketList';
    total: number;
    data: Array<{
      __typename?: 'BucketInfo';
      name: string;
      createTime: number;
      updateTime: number;
      access: BucketAccess;
    }>;
  };
};

export type BucketInfoQueryVariables = Exact<{
  data: GetBucketRequest;
}>;

export type BucketInfoQuery = {
  __typename?: 'QueryRoot';
  bucketInfo: { __typename?: 'BucketInfo'; name: string; updateTime: number; createTime: number; access: BucketAccess };
};

export type CreateBucketMutationVariables = Exact<{
  data: CreateBucketRequest;
}>;

export type CreateBucketMutation = {
  __typename?: 'MutationRoot';
  createBucket: { __typename?: 'BucketInfo'; name: string };
};

export type UpdateBucketMutationVariables = Exact<{
  data: UpdateBucketRequest;
}>;

export type UpdateBucketMutation = {
  __typename?: 'MutationRoot';
  updateBucket: { __typename?: 'BucketInfo'; name: string };
};

export type DeleteBucketMutationVariables = Exact<{
  data: DeleteBucketRequest;
}>;

export type DeleteBucketMutation = { __typename?: 'MutationRoot'; deleteBucket: boolean };

export type CreateFolderMutationVariables = Exact<{
  data: CreateFolderRequest;
}>;

export type CreateFolderMutation = {
  __typename?: 'MutationRoot';
  createFolder: { __typename?: 'FolderInfo'; path: string };
};

export type UpdateFolderMutationVariables = Exact<{
  data: UpdateFolderRequest;
}>;

export type UpdateFolderMutation = {
  __typename?: 'MutationRoot';
  updateFolder: { __typename?: 'FolderInfo'; path: string };
};

export type DeleteFolderMutationVariables = Exact<{
  data: DeleteFolderRequest;
}>;

export type DeleteFolderMutation = { __typename?: 'MutationRoot'; deleteFolder: boolean };

export type FolderListQueryVariables = Exact<{
  data: GetFolderListRequest;
}>;

export type FolderListQuery = {
  __typename?: 'QueryRoot';
  folderList: {
    __typename?: 'FolderList';
    total: number;
    data: Array<
      | {
          __typename: 'FolderInfo';
          path: string;
          createTime: number;
          updateTime: number;
          bucketName: string;
          access: FolderAccess;
          fatherPath: string;
          folderName: string;
        }
      | {
          __typename: 'ObjectInfo';
          path: string;
          filename: string;
          bucketName: string;
          createTime: number;
          updateTime: number;
          size: number;
          objectAccess: ObjectAccess;
        }
    >;
  };
};

export type FolderStatisticsQueryVariables = Exact<{
  data: GetFolderRequest;
}>;

export type FolderStatisticsQuery = {
  __typename?: 'QueryRoot';
  folderInfo: { __typename?: 'FolderInfo'; folderCount: number; objectCount: number; objectSize: number };
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
      description?: string | null;
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
    description?: string | null;
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

export type UpdateObjectMutationVariables = Exact<{
  data: UpdateObjectRequest;
}>;

export type UpdateObjectMutation = {
  __typename?: 'MutationRoot';
  updateObject: { __typename?: 'ObjectInfo'; filename: string };
};

export type DeleteObjectMutationVariables = Exact<{
  data: DeleteObjectRequest;
}>;

export type DeleteObjectMutation = { __typename?: 'MutationRoot'; deleteObject: boolean };

export type GetObjectQueryVariables = Exact<{
  data: GetObjectRequest;
}>;

export type GetObjectQuery = {
  __typename?: 'QueryRoot';
  objectInfo: {
    __typename?: 'ObjectInfo';
    filename: string;
    blake3: string;
    size: number;
    updateTime: number;
    bucketName: string;
    access: ObjectAccess;
    path: string;
    headers: Array<{ __typename?: 'Header'; key: string; value: string }>;
  };
};

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
    description?: string | null;
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

export const BucketListDocument = gql`
  query bucketList($data: GetListRequest!) {
    bucketList(data: $data) {
      data {
        name
        createTime
        updateTime
        access
      }
      total
    }
  }
`;

/**
 * __useBucketListQuery__
 *
 * To run a query within a React component, call `useBucketListQuery` and pass it any options that fit your needs.
 * When your component renders, `useBucketListQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useBucketListQuery({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useBucketListQuery(baseOptions: Apollo.QueryHookOptions<BucketListQuery, BucketListQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<BucketListQuery, BucketListQueryVariables>(BucketListDocument, options);
}
export function useBucketListLazyQuery(
  baseOptions?: Apollo.LazyQueryHookOptions<BucketListQuery, BucketListQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<BucketListQuery, BucketListQueryVariables>(BucketListDocument, options);
}
export type BucketListQueryHookResult = ReturnType<typeof useBucketListQuery>;
export type BucketListLazyQueryHookResult = ReturnType<typeof useBucketListLazyQuery>;
export type BucketListQueryResult = Apollo.QueryResult<BucketListQuery, BucketListQueryVariables>;
export const BucketInfoDocument = gql`
  query bucketInfo($data: GetBucketRequest!) {
    bucketInfo(data: $data) {
      name
      updateTime
      createTime
      access
    }
  }
`;

/**
 * __useBucketInfoQuery__
 *
 * To run a query within a React component, call `useBucketInfoQuery` and pass it any options that fit your needs.
 * When your component renders, `useBucketInfoQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useBucketInfoQuery({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useBucketInfoQuery(baseOptions: Apollo.QueryHookOptions<BucketInfoQuery, BucketInfoQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<BucketInfoQuery, BucketInfoQueryVariables>(BucketInfoDocument, options);
}
export function useBucketInfoLazyQuery(
  baseOptions?: Apollo.LazyQueryHookOptions<BucketInfoQuery, BucketInfoQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<BucketInfoQuery, BucketInfoQueryVariables>(BucketInfoDocument, options);
}
export type BucketInfoQueryHookResult = ReturnType<typeof useBucketInfoQuery>;
export type BucketInfoLazyQueryHookResult = ReturnType<typeof useBucketInfoLazyQuery>;
export type BucketInfoQueryResult = Apollo.QueryResult<BucketInfoQuery, BucketInfoQueryVariables>;
export const CreateBucketDocument = gql`
  mutation createBucket($data: CreateBucketRequest!) {
    createBucket(data: $data) {
      name
    }
  }
`;
export type CreateBucketMutationFn = Apollo.MutationFunction<CreateBucketMutation, CreateBucketMutationVariables>;

/**
 * __useCreateBucketMutation__
 *
 * To run a mutation, you first call `useCreateBucketMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useCreateBucketMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [createBucketMutation, { data, loading, error }] = useCreateBucketMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useCreateBucketMutation(
  baseOptions?: Apollo.MutationHookOptions<CreateBucketMutation, CreateBucketMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<CreateBucketMutation, CreateBucketMutationVariables>(CreateBucketDocument, options);
}
export type CreateBucketMutationHookResult = ReturnType<typeof useCreateBucketMutation>;
export type CreateBucketMutationResult = Apollo.MutationResult<CreateBucketMutation>;
export type CreateBucketMutationOptions = Apollo.BaseMutationOptions<
  CreateBucketMutation,
  CreateBucketMutationVariables
>;
export const UpdateBucketDocument = gql`
  mutation updateBucket($data: UpdateBucketRequest!) {
    updateBucket(data: $data) {
      name
    }
  }
`;
export type UpdateBucketMutationFn = Apollo.MutationFunction<UpdateBucketMutation, UpdateBucketMutationVariables>;

/**
 * __useUpdateBucketMutation__
 *
 * To run a mutation, you first call `useUpdateBucketMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useUpdateBucketMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [updateBucketMutation, { data, loading, error }] = useUpdateBucketMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUpdateBucketMutation(
  baseOptions?: Apollo.MutationHookOptions<UpdateBucketMutation, UpdateBucketMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<UpdateBucketMutation, UpdateBucketMutationVariables>(UpdateBucketDocument, options);
}
export type UpdateBucketMutationHookResult = ReturnType<typeof useUpdateBucketMutation>;
export type UpdateBucketMutationResult = Apollo.MutationResult<UpdateBucketMutation>;
export type UpdateBucketMutationOptions = Apollo.BaseMutationOptions<
  UpdateBucketMutation,
  UpdateBucketMutationVariables
>;
export const DeleteBucketDocument = gql`
  mutation deleteBucket($data: DeleteBucketRequest!) {
    deleteBucket(data: $data)
  }
`;
export type DeleteBucketMutationFn = Apollo.MutationFunction<DeleteBucketMutation, DeleteBucketMutationVariables>;

/**
 * __useDeleteBucketMutation__
 *
 * To run a mutation, you first call `useDeleteBucketMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useDeleteBucketMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [deleteBucketMutation, { data, loading, error }] = useDeleteBucketMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useDeleteBucketMutation(
  baseOptions?: Apollo.MutationHookOptions<DeleteBucketMutation, DeleteBucketMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<DeleteBucketMutation, DeleteBucketMutationVariables>(DeleteBucketDocument, options);
}
export type DeleteBucketMutationHookResult = ReturnType<typeof useDeleteBucketMutation>;
export type DeleteBucketMutationResult = Apollo.MutationResult<DeleteBucketMutation>;
export type DeleteBucketMutationOptions = Apollo.BaseMutationOptions<
  DeleteBucketMutation,
  DeleteBucketMutationVariables
>;
export const CreateFolderDocument = gql`
  mutation createFolder($data: CreateFolderRequest!) {
    createFolder(data: $data) {
      path
    }
  }
`;
export type CreateFolderMutationFn = Apollo.MutationFunction<CreateFolderMutation, CreateFolderMutationVariables>;

/**
 * __useCreateFolderMutation__
 *
 * To run a mutation, you first call `useCreateFolderMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useCreateFolderMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [createFolderMutation, { data, loading, error }] = useCreateFolderMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useCreateFolderMutation(
  baseOptions?: Apollo.MutationHookOptions<CreateFolderMutation, CreateFolderMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<CreateFolderMutation, CreateFolderMutationVariables>(CreateFolderDocument, options);
}
export type CreateFolderMutationHookResult = ReturnType<typeof useCreateFolderMutation>;
export type CreateFolderMutationResult = Apollo.MutationResult<CreateFolderMutation>;
export type CreateFolderMutationOptions = Apollo.BaseMutationOptions<
  CreateFolderMutation,
  CreateFolderMutationVariables
>;
export const UpdateFolderDocument = gql`
  mutation updateFolder($data: UpdateFolderRequest!) {
    updateFolder(data: $data) {
      path
    }
  }
`;
export type UpdateFolderMutationFn = Apollo.MutationFunction<UpdateFolderMutation, UpdateFolderMutationVariables>;

/**
 * __useUpdateFolderMutation__
 *
 * To run a mutation, you first call `useUpdateFolderMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useUpdateFolderMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [updateFolderMutation, { data, loading, error }] = useUpdateFolderMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUpdateFolderMutation(
  baseOptions?: Apollo.MutationHookOptions<UpdateFolderMutation, UpdateFolderMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<UpdateFolderMutation, UpdateFolderMutationVariables>(UpdateFolderDocument, options);
}
export type UpdateFolderMutationHookResult = ReturnType<typeof useUpdateFolderMutation>;
export type UpdateFolderMutationResult = Apollo.MutationResult<UpdateFolderMutation>;
export type UpdateFolderMutationOptions = Apollo.BaseMutationOptions<
  UpdateFolderMutation,
  UpdateFolderMutationVariables
>;
export const DeleteFolderDocument = gql`
  mutation deleteFolder($data: DeleteFolderRequest!) {
    deleteFolder(data: $data)
  }
`;
export type DeleteFolderMutationFn = Apollo.MutationFunction<DeleteFolderMutation, DeleteFolderMutationVariables>;

/**
 * __useDeleteFolderMutation__
 *
 * To run a mutation, you first call `useDeleteFolderMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useDeleteFolderMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [deleteFolderMutation, { data, loading, error }] = useDeleteFolderMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useDeleteFolderMutation(
  baseOptions?: Apollo.MutationHookOptions<DeleteFolderMutation, DeleteFolderMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<DeleteFolderMutation, DeleteFolderMutationVariables>(DeleteFolderDocument, options);
}
export type DeleteFolderMutationHookResult = ReturnType<typeof useDeleteFolderMutation>;
export type DeleteFolderMutationResult = Apollo.MutationResult<DeleteFolderMutation>;
export type DeleteFolderMutationOptions = Apollo.BaseMutationOptions<
  DeleteFolderMutation,
  DeleteFolderMutationVariables
>;
export const FolderListDocument = gql`
  query folderList($data: GetFolderListRequest!) {
    folderList(data: $data) {
      total
      data {
        ... on FolderInfo {
          path
          createTime
          updateTime
          bucketName
          access
          fatherPath
          folderName
          __typename
        }
        ... on ObjectInfo {
          path
          filename
          bucketName
          objectAccess: access
          createTime
          updateTime
          size
          __typename
        }
      }
    }
  }
`;

/**
 * __useFolderListQuery__
 *
 * To run a query within a React component, call `useFolderListQuery` and pass it any options that fit your needs.
 * When your component renders, `useFolderListQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useFolderListQuery({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useFolderListQuery(baseOptions: Apollo.QueryHookOptions<FolderListQuery, FolderListQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<FolderListQuery, FolderListQueryVariables>(FolderListDocument, options);
}
export function useFolderListLazyQuery(
  baseOptions?: Apollo.LazyQueryHookOptions<FolderListQuery, FolderListQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<FolderListQuery, FolderListQueryVariables>(FolderListDocument, options);
}
export type FolderListQueryHookResult = ReturnType<typeof useFolderListQuery>;
export type FolderListLazyQueryHookResult = ReturnType<typeof useFolderListLazyQuery>;
export type FolderListQueryResult = Apollo.QueryResult<FolderListQuery, FolderListQueryVariables>;
export const FolderStatisticsDocument = gql`
  query folderStatistics($data: GetFolderRequest!) {
    folderInfo(data: $data) {
      folderCount
      objectCount
      objectSize
    }
  }
`;

/**
 * __useFolderStatisticsQuery__
 *
 * To run a query within a React component, call `useFolderStatisticsQuery` and pass it any options that fit your needs.
 * When your component renders, `useFolderStatisticsQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useFolderStatisticsQuery({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useFolderStatisticsQuery(
  baseOptions: Apollo.QueryHookOptions<FolderStatisticsQuery, FolderStatisticsQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<FolderStatisticsQuery, FolderStatisticsQueryVariables>(FolderStatisticsDocument, options);
}
export function useFolderStatisticsLazyQuery(
  baseOptions?: Apollo.LazyQueryHookOptions<FolderStatisticsQuery, FolderStatisticsQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<FolderStatisticsQuery, FolderStatisticsQueryVariables>(FolderStatisticsDocument, options);
}
export type FolderStatisticsQueryHookResult = ReturnType<typeof useFolderStatisticsQuery>;
export type FolderStatisticsLazyQueryHookResult = ReturnType<typeof useFolderStatisticsLazyQuery>;
export type FolderStatisticsQueryResult = Apollo.QueryResult<FolderStatisticsQuery, FolderStatisticsQueryVariables>;
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
export const UpdateObjectDocument = gql`
  mutation updateObject($data: UpdateObjectRequest!) {
    updateObject(data: $data) {
      filename
    }
  }
`;
export type UpdateObjectMutationFn = Apollo.MutationFunction<UpdateObjectMutation, UpdateObjectMutationVariables>;

/**
 * __useUpdateObjectMutation__
 *
 * To run a mutation, you first call `useUpdateObjectMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useUpdateObjectMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [updateObjectMutation, { data, loading, error }] = useUpdateObjectMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUpdateObjectMutation(
  baseOptions?: Apollo.MutationHookOptions<UpdateObjectMutation, UpdateObjectMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<UpdateObjectMutation, UpdateObjectMutationVariables>(UpdateObjectDocument, options);
}
export type UpdateObjectMutationHookResult = ReturnType<typeof useUpdateObjectMutation>;
export type UpdateObjectMutationResult = Apollo.MutationResult<UpdateObjectMutation>;
export type UpdateObjectMutationOptions = Apollo.BaseMutationOptions<
  UpdateObjectMutation,
  UpdateObjectMutationVariables
>;
export const DeleteObjectDocument = gql`
  mutation deleteObject($data: DeleteObjectRequest!) {
    deleteObject(data: $data)
  }
`;
export type DeleteObjectMutationFn = Apollo.MutationFunction<DeleteObjectMutation, DeleteObjectMutationVariables>;

/**
 * __useDeleteObjectMutation__
 *
 * To run a mutation, you first call `useDeleteObjectMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useDeleteObjectMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [deleteObjectMutation, { data, loading, error }] = useDeleteObjectMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useDeleteObjectMutation(
  baseOptions?: Apollo.MutationHookOptions<DeleteObjectMutation, DeleteObjectMutationVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useMutation<DeleteObjectMutation, DeleteObjectMutationVariables>(DeleteObjectDocument, options);
}
export type DeleteObjectMutationHookResult = ReturnType<typeof useDeleteObjectMutation>;
export type DeleteObjectMutationResult = Apollo.MutationResult<DeleteObjectMutation>;
export type DeleteObjectMutationOptions = Apollo.BaseMutationOptions<
  DeleteObjectMutation,
  DeleteObjectMutationVariables
>;
export const GetObjectDocument = gql`
  query getObject($data: GetObjectRequest!) {
    objectInfo(data: $data) {
      filename
      blake3
      size
      updateTime
      bucketName
      access
      headers {
        key
        value
      }
      path
    }
  }
`;

/**
 * __useGetObjectQuery__
 *
 * To run a query within a React component, call `useGetObjectQuery` and pass it any options that fit your needs.
 * When your component renders, `useGetObjectQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGetObjectQuery({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useGetObjectQuery(baseOptions: Apollo.QueryHookOptions<GetObjectQuery, GetObjectQueryVariables>) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useQuery<GetObjectQuery, GetObjectQueryVariables>(GetObjectDocument, options);
}
export function useGetObjectLazyQuery(
  baseOptions?: Apollo.LazyQueryHookOptions<GetObjectQuery, GetObjectQueryVariables>,
) {
  const options = { ...defaultOptions, ...baseOptions };
  return Apollo.useLazyQuery<GetObjectQuery, GetObjectQueryVariables>(GetObjectDocument, options);
}
export type GetObjectQueryHookResult = ReturnType<typeof useGetObjectQuery>;
export type GetObjectLazyQueryHookResult = ReturnType<typeof useGetObjectLazyQuery>;
export type GetObjectQueryResult = Apollo.QueryResult<GetObjectQuery, GetObjectQueryVariables>;
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
