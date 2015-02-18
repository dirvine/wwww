/* Copyright 2015 David Irvine <david.irvine@maidsafe.net>
    This file is part of wwww

    wwww is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    wwww is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with wwww (file called COPYING in project root directory).
    If not, see <http://www.gnu.org/licenses/>. */
#pragma once

#include <memory> /* for std::unique_ptr */

struct quad {
  std::string subject;   /** The subject term. */
  std::string predicate; /** The predicate term. */
  std::string object;    /** The object term. */
  std::string context;   /** The context/graph term. */

  /**
   * Default constructor.
   */
  // quad() noexcept {}

  /**
   * Destructor.
   */
  // ~quad() noexcept = default;

  /**
   * Sets all terms to `nullptr`.
   */

  // inline bool has_context() const noexcept { return context.get() != nullptr; }
  //
  // inline bool has_subject() const noexcept { return subject.get() != nullptr; }
  //
  // inline bool has_predicate() const noexcept { return predicate.get() != nullptr; }
  //
  // inline bool has_object() const noexcept { return object.get() != nullptr; }
  //
  // inline const std::unique_ptr<term>& operator[](const std::size_t index) const {
  //   switch (index) {
  //     case 0:
  //       return context;
  //     case 1:
  //       return subject;
  //     case 2:
  //       return predicate;
  //     case 3:
  //       return object;
  //     default:
  //       throw std::out_of_range("subscript must be 0..3");
  //   }
  // }
  //
  // inline std::unique_ptr<term>& operator[](const std::size_t index) {
  //   switch (index) {
  //     case 0:
  //       return context;
  //     case 1:
  //       return subject;
  //     case 2:
  //       return predicate;
  //     case 3:
  //       return object;
  //     default:
  //       throw std::out_of_range("subscript must be 0..3");
  //   }
  // }
};
