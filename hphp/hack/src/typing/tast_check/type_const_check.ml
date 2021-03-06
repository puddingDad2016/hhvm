(*
 * Copyright (c) 2019, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)

open Hh_prelude
open Aast
open Typing_defs
module Cls = Decl_provider.Class
module Env = Tast_env
open Typing_const_reifiable

let handler =
  object
    inherit Tast_visitor.handler_base

    method! at_class_typeconst
        env { c_tconst_abstract; c_tconst_name = (p, name); _ } =
      Option.(
        let cls_opt = Tast_env.get_self_id env >>= Tast_env.get_class env in
        match cls_opt with
        | None -> ()
        | Some cls ->
          begin
            match (Cls.kind cls, c_tconst_abstract) with
            | (Ast_defs.Cnormal, TCAbstract _) ->
              Errors.implement_abstract
                ~is_final:(Cls.final cls)
                (Cls.pos cls)
                p
                "type constant"
                name
            | _ -> ()
          end;
          begin
            match Cls.get_typeconst cls name with
            | None -> ()
            | Some tc ->
              begin
                match (tc.ttc_abstract, tc.ttc_type) with
                | (TCAbstract (Some ty), _)
                | ((TCPartiallyAbstract | TCConcrete), Some ty) ->
                  if snd tc.ttc_enforceable then
                    let pos = fst tc.ttc_enforceable in
                    Typing_enforceable_hint.validate_type
                      env
                      (fst tc.ttc_name)
                      ty
                      (Errors.invalid_enforceable_type "constant" (pos, name))
                | _ -> ()
              end;
              if String.equal tc.ttc_origin (Cls.name cls) then
                Option.iter tc.ttc_reifiable (check_reifiable env tc)
          end)
  end
