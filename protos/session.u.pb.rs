const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Session_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Session {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Session>
}

impl ::protobuf::Message for Session {}

impl ::std::default::Default for Session {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Session {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Session` is `Sync` because it does not implement interior mutability.
//    Neither does `SessionMut`.
unsafe impl Sync for Session {}

// SAFETY:
// - `Session` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Session {}

impl ::protobuf::Proxied for Session {
  type View<'msg> = SessionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Session {}

impl ::protobuf::MutProxied for Session {
  type Mut<'msg> = SessionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SessionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Session>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SessionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SessionView<'msg> {
  type Message = Session;
}

impl ::std::fmt::Debug for SessionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SessionView<'_> {
  fn default() -> SessionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Session>> for SessionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Session>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SessionView<'msg> {

  pub fn to_owned(&self) -> Session {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key: optional bytes
  pub fn has_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn key_opt(self) -> ::protobuf::Optional<&'msg [u8]> {
        ::protobuf::Optional::new(self.key(), self.has_key())
  }
  pub fn key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // secret: optional bytes
  pub fn has_secret(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn secret_opt(self) -> ::protobuf::Optional<&'msg [u8]> {
        ::protobuf::Optional::new(self.secret(), self.has_secret())
  }
  pub fn secret(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // previous_secret: optional bytes
  pub fn has_previous_secret(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn previous_secret_opt(self) -> ::protobuf::Optional<&'msg [u8]> {
        ::protobuf::Optional::new(self.previous_secret(), self.has_previous_secret())
  }
  pub fn previous_secret(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // user_name: optional string
  pub fn has_user_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn user_name_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.user_name(), self.has_user_name())
  }
  pub fn user_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // expire_time: optional message google.protobuf.Timestamp
  pub fn has_expire_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn expire_time_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.expire_time(), self.has_expire_time())
  }
  pub fn expire_time(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // rotation_time: optional message google.protobuf.Timestamp
  pub fn has_rotation_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn rotation_time_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.rotation_time(), self.has_rotation_time())
  }
  pub fn rotation_time(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // previous_secret_expire_time: optional message google.protobuf.Timestamp
  pub fn has_previous_secret_expire_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn previous_secret_expire_time_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.previous_secret_expire_time(), self.has_previous_secret_expire_time())
  }
  pub fn previous_secret_expire_time(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // is_admin: optional bool
  pub fn has_is_admin(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
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
        7, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SessionView` is `Sync` because it does not support mutation.
unsafe impl Sync for SessionView<'_> {}

// SAFETY:
// - `SessionView` is `Send` because while its alive a `SessionMut` cannot.
// - `SessionView` does not use thread-local data.
unsafe impl Send for SessionView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SessionView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for SessionView<'msg> {}

impl<'msg> ::protobuf::AsView for SessionView<'msg> {
  type Proxied = Session;
  fn as_view(&self) -> ::protobuf::View<'msg, Session> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SessionView<'msg> {
  fn into_view<'shorter>(self) -> SessionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Session> for SessionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Session {
    let mut dst = Session::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Session> for SessionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Session {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Session {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SessionView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SessionMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SessionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Session>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SessionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SessionMut<'msg> {
  type Message = Session;
}

impl ::std::fmt::Debug for SessionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Session>> for SessionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Session>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SessionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Session> {
    self.inner
  }

  pub fn to_owned(&self) -> Session {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key: optional bytes
  pub fn has_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.key(), self.has_key())
  }
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
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

  // secret: optional bytes
  pub fn has_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn secret_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.secret(), self.has_secret())
  }
  pub fn secret(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_secret(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
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

  // previous_secret: optional bytes
  pub fn has_previous_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_previous_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn previous_secret_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.previous_secret(), self.has_previous_secret())
  }
  pub fn previous_secret(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_previous_secret(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
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

  // user_name: optional string
  pub fn has_user_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_user_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn user_name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.user_name(), self.has_user_name())
  }
  pub fn user_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
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
        3,
        view,
      );
    }
  }

  // expire_time: optional message google.protobuf.Timestamp
  pub fn has_expire_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_expire_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn expire_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.expire_time(), self.has_expire_time())
  }
  pub fn expire_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn expire_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_expire_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // rotation_time: optional message google.protobuf.Timestamp
  pub fn has_rotation_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_rotation_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn rotation_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.rotation_time(), self.has_rotation_time())
  }
  pub fn rotation_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn rotation_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_rotation_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // previous_secret_expire_time: optional message google.protobuf.Timestamp
  pub fn has_previous_secret_expire_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_previous_secret_expire_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn previous_secret_expire_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.previous_secret_expire_time(), self.has_previous_secret_expire_time())
  }
  pub fn previous_secret_expire_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn previous_secret_expire_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_previous_secret_expire_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // is_admin: optional bool
  pub fn has_is_admin(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_is_admin(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
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
        7, (false).into()
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
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `SessionMut` does not perform any shared mutation.
unsafe impl Send for SessionMut<'_> {}

// SAFETY:
// - `SessionMut` does not perform any shared mutation.
unsafe impl Sync for SessionMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SessionMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for SessionMut<'msg> {}

impl<'msg> ::protobuf::AsView for SessionMut<'msg> {
  type Proxied = Session;
  fn as_view(&self) -> ::protobuf::View<'_, Session> {
    SessionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SessionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Session>
  where
      'msg: 'shorter {
    SessionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for SessionMut<'msg> {
  type MutProxied = Session;
  fn as_mut(&mut self) -> SessionMut<'msg> {
    SessionMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SessionMut<'msg> {
  fn into_mut<'shorter>(self) -> SessionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Session {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Session> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SessionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SessionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key: optional bytes
  pub fn has_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.key(), self.has_key())
  }
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
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

  // secret: optional bytes
  pub fn has_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn secret_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.secret(), self.has_secret())
  }
  pub fn secret(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_secret(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
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

  // previous_secret: optional bytes
  pub fn has_previous_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_previous_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn previous_secret_opt(&self) -> ::protobuf::Optional<&'_ [u8]> {
        ::protobuf::Optional::new(self.previous_secret(), self.has_previous_secret())
  }
  pub fn previous_secret(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_previous_secret(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
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

  // user_name: optional string
  pub fn has_user_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_user_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn user_name_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.user_name(), self.has_user_name())
  }
  pub fn user_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
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
        3,
        view,
      );
    }
  }

  // expire_time: optional message google.protobuf.Timestamp
  pub fn has_expire_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_expire_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn expire_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.expire_time(), self.has_expire_time())
  }
  pub fn expire_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn expire_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_expire_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // rotation_time: optional message google.protobuf.Timestamp
  pub fn has_rotation_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_rotation_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn rotation_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.rotation_time(), self.has_rotation_time())
  }
  pub fn rotation_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn rotation_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_rotation_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // previous_secret_expire_time: optional message google.protobuf.Timestamp
  pub fn has_previous_secret_expire_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_previous_secret_expire_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn previous_secret_expire_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.previous_secret_expire_time(), self.has_previous_secret_expire_time())
  }
  pub fn previous_secret_expire_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn previous_secret_expire_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_previous_secret_expire_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // is_admin: optional bool
  pub fn has_is_admin(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_is_admin(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
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
        7, (false).into()
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
        7, val.into()
      )
    }
  }

}  // impl Session

impl ::std::ops::Drop for Session {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Session {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Session {
  type Proxied = Self;
  fn as_view(&self) -> SessionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Session {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SessionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Session {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::Session_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0001333/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::Session_msg_init.0, &[<::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::Session_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Session {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Session {
  type Msg = Session;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Session> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Session {
  type Msg = Session;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Session> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SessionMut<'_> {
  type Msg = Session;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Session> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SessionMut<'_> {
  type Msg = Session;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Session> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SessionView<'_> {
  type Msg = Session;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Session> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SessionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



