const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut User_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct User {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<User>
}

impl ::protobuf::Message for User {}

impl ::std::default::Default for User {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for User {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `User` is `Sync` because it does not implement interior mutability.
//    Neither does `UserMut`.
unsafe impl Sync for User {}

// SAFETY:
// - `User` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for User {}

impl ::protobuf::Proxied for User {
  type View<'msg> = UserView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for User {}

impl ::protobuf::MutProxied for User {
  type Mut<'msg> = UserMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UserView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, User>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UserView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UserView<'msg> {
  type Message = User;
}

impl ::std::fmt::Debug for UserView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UserView<'_> {
  fn default() -> UserView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, User>> for UserView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, User>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UserView<'msg> {

  pub fn to_owned(&self) -> User {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // user_name: optional string
  pub fn has_user_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn user_name_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.user_name(), self.has_user_name())
  }
  pub fn user_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // email: optional string
  pub fn has_email(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn email_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.email(), self.has_email())
  }
  pub fn email(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // password_sha256: optional bytes
  pub fn has_password_sha256(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn password_sha256_opt(self) -> ::protobuf::Optional<&'msg [u8]> {
        ::protobuf::Optional::new(self.password_sha256(), self.has_password_sha256())
  }
  pub fn password_sha256(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // password_salt: optional bytes
  pub fn has_password_salt(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn password_salt_opt(self) -> ::protobuf::Optional<&'msg [u8]> {
        ::protobuf::Optional::new(self.password_salt(), self.has_password_salt())
  }
  pub fn password_salt(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // is_admin: optional bool
  pub fn has_is_admin(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn is_admin_opt(self) -> ::protobuf::Optional<bool> {
        ::protobuf::Optional::new(self.is_admin(), self.has_is_admin())
  }
  pub fn is_admin(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `UserView` is `Sync` because it does not support mutation.
unsafe impl Sync for UserView<'_> {}

// SAFETY:
// - `UserView` is `Send` because while its alive a `UserMut` cannot.
// - `UserView` does not use thread-local data.
unsafe impl Send for UserView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for UserView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for UserView<'msg> {}

impl<'msg> ::protobuf::AsView for UserView<'msg> {
  type Proxied = User;
  fn as_view(&self) -> ::protobuf::View<'msg, User> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UserView<'msg> {
  fn into_view<'shorter>(self) -> UserView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<User> for UserView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> User {
    let mut dst = User::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<User> for UserMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> User {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for User {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UserView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UserMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UserMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, User>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UserMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UserMut<'msg> {
  type Message = User;
}

impl ::std::fmt::Debug for UserMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, User>> for UserMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, User>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UserMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, User> {
    self.inner
  }

  pub fn to_owned(&self) -> User {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // user_name: optional string
  pub fn has_user_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_user_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn user_name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.user_name(), self.has_user_name())
  }
  pub fn user_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_user_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        0,
        view,
      );
    }
  }

  // email: optional string
  pub fn has_email(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_email(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn email_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.email(), self.has_email())
  }
  pub fn email(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_email(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

  // password_sha256: optional bytes
  pub fn has_password_sha256(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_password_sha256(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn password_sha256_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.password_sha256(), self.has_password_sha256())
  }
  pub fn password_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_password_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        2,
        view,
      );
    }
  }

  // password_salt: optional bytes
  pub fn has_password_salt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_password_salt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn password_salt_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.password_salt(), self.has_password_salt())
  }
  pub fn password_salt(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_password_salt(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        3,
        view,
      );
    }
  }

  // is_admin: optional bool
  pub fn has_is_admin(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_is_admin(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn is_admin_opt(&self) -> ::protobuf::Optional<bool> {
        ::protobuf::Optional::new(self.is_admin(), self.has_is_admin())
  }
  pub fn is_admin(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_admin(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

}

// SAFETY:
// - `UserMut` does not perform any shared mutation.
unsafe impl Send for UserMut<'_> {}

// SAFETY:
// - `UserMut` does not perform any shared mutation.
unsafe impl Sync for UserMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for UserMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for UserMut<'msg> {}

impl<'msg> ::protobuf::AsView for UserMut<'msg> {
  type Proxied = User;
  fn as_view(&self) -> ::protobuf::View<'_, User> {
    UserView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UserMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, User>
  where
      'msg: 'shorter {
    UserView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for UserMut<'msg> {
  type MutProxied = User;
  fn as_mut(&mut self) -> UserMut<'msg> {
    UserMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UserMut<'msg> {
  fn into_mut<'shorter>(self) -> UserMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl User {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, User> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UserView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UserMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // user_name: optional string
  pub fn has_user_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_user_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn user_name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.user_name(), self.has_user_name())
  }
  pub fn user_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_user_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        0,
        view,
      );
    }
  }

  // email: optional string
  pub fn has_email(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_email(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn email_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.email(), self.has_email())
  }
  pub fn email(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_email(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

  // password_sha256: optional bytes
  pub fn has_password_sha256(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_password_sha256(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn password_sha256_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.password_sha256(), self.has_password_sha256())
  }
  pub fn password_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_password_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        2,
        view,
      );
    }
  }

  // password_salt: optional bytes
  pub fn has_password_salt(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_password_salt(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn password_salt_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.password_salt(), self.has_password_salt())
  }
  pub fn password_salt(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_password_salt(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        3,
        view,
      );
    }
  }

  // is_admin: optional bool
  pub fn has_is_admin(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_is_admin(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn is_admin_opt(&self) -> ::protobuf::Optional<bool> {
        ::protobuf::Optional::new(self.is_admin(), self.has_is_admin())
  }
  pub fn is_admin(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_admin(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

}  // impl User

impl ::std::ops::Drop for User {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for User {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for User {
  type Proxied = Self;
  fn as_view(&self) -> UserView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for User {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UserMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for User {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::User_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1100/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::User_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::User_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for User {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for User {
  type Msg = User;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<User> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for User {
  type Msg = User;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<User> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UserMut<'_> {
  type Msg = User;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<User> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UserMut<'_> {
  type Msg = User;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<User> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UserView<'_> {
  type Msg = User;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<User> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UserMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



