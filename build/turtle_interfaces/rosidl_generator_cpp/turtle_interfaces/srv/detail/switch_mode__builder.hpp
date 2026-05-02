// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from turtle_interfaces:srv/SwitchMode.idl
// generated code does not contain a copyright notice

#ifndef TURTLE_INTERFACES__SRV__DETAIL__SWITCH_MODE__BUILDER_HPP_
#define TURTLE_INTERFACES__SRV__DETAIL__SWITCH_MODE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "turtle_interfaces/srv/detail/switch_mode__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace turtle_interfaces
{

namespace srv
{

namespace builder
{

class Init_SwitchMode_Request_mode
{
public:
  Init_SwitchMode_Request_mode()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::turtle_interfaces::srv::SwitchMode_Request mode(::turtle_interfaces::srv::SwitchMode_Request::_mode_type arg)
  {
    msg_.mode = std::move(arg);
    return std::move(msg_);
  }

private:
  ::turtle_interfaces::srv::SwitchMode_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::turtle_interfaces::srv::SwitchMode_Request>()
{
  return turtle_interfaces::srv::builder::Init_SwitchMode_Request_mode();
}

}  // namespace turtle_interfaces


namespace turtle_interfaces
{

namespace srv
{

namespace builder
{

class Init_SwitchMode_Response_message
{
public:
  explicit Init_SwitchMode_Response_message(::turtle_interfaces::srv::SwitchMode_Response & msg)
  : msg_(msg)
  {}
  ::turtle_interfaces::srv::SwitchMode_Response message(::turtle_interfaces::srv::SwitchMode_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::turtle_interfaces::srv::SwitchMode_Response msg_;
};

class Init_SwitchMode_Response_success
{
public:
  Init_SwitchMode_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SwitchMode_Response_message success(::turtle_interfaces::srv::SwitchMode_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SwitchMode_Response_message(msg_);
  }

private:
  ::turtle_interfaces::srv::SwitchMode_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::turtle_interfaces::srv::SwitchMode_Response>()
{
  return turtle_interfaces::srv::builder::Init_SwitchMode_Response_success();
}

}  // namespace turtle_interfaces

#endif  // TURTLE_INTERFACES__SRV__DETAIL__SWITCH_MODE__BUILDER_HPP_
