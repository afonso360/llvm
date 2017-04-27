use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use super::*;

macro_rules! inst_list {
    (@inner $bfn: ident, $build_fn: path, $($argn: ident : $argty: ty),*) => {
        impl Builder {
            // We might not always return valueref
            pub fn $bfn(&mut self, $($argn: $argty),*) -> LLVMValueRef {
                unsafe {
                    $build_fn(self.ptr, $($argn),*)
                }
            }
        }
    };
    (@inner $bfn: ident, $($argn: ident : $argty: ty),* => $b: block) => {
        impl Builder {
            // We might not always return valueref
            pub fn $bfn(&mut self, $($argn: $argty),*) -> LLVMValueRef { unsafe { $b } }
        }
    };
    (@inner $bfn: ident, $build_fn: path) => {
        impl Builder {
            // We might not always return valueref
            pub fn $bfn(&mut self) -> LLVMValueRef { unsafe { $build_fn(self.ptr) } }
        }
    };
    ({ $(($bfn: ident, $($rest:tt)*))* }) => {
        $(
            inst_list!(@inner $bfn, $($rest)*);
        )*
    }
}

pub struct Builder {
    pub ptr: LLVMBuilderRef
}
impl_llvm_ref!(Builder, LLVMBuilderRef);

//add_inst!("retvoid", build_ret_void, llvm::LLVMBuildRetVoid);
inst_list!({
    (build_ret,       llvm::LLVMBuildRet,       ret_val: LLVMValueRef)

    (build_ret_void,  llvm::LLVMBuildRetVoid)

    (build_store,     llvm::LLVMBuildStore,     val: LLVMValueRef,
                                                ptr: LLVMValueRef)

    (build_br,        llvm::LLVMBuildBr,        dest: LLVMBasicBlockRef)

    (build_cond_br,   llvm::LLVMBuildCondBr,    cond: LLVMValueRef,
                                                then: LLVMBasicBlockRef,
                                                else_: LLVMBasicBlockRef)
});

impl Builder {
    pub fn position_at_end(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            llvm::LLVMPositionBuilderAtEnd(self.ptr, basic_block);
        }
    }

    pub fn build_alloca(&mut self, ty: LLVMTypeRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildAlloca(self.ptr, ty, c_name.as_ptr())
        }
    }


    pub fn build_load(&mut self, ptr: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildLoad(self.ptr, ptr, c_name.as_ptr())
        }
    }

    pub fn build_call(&mut self, func: Function, mut args: Vec<LLVMValueRef>,
                      name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildCall(
                self.ptr,
                func.ptr,
                args.as_mut_ptr(),
                args.len() as u32,
                c_name.as_ptr()
            )
        }
    }

    pub fn build_add(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildAdd(self.ptr, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_sub(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildSub(self.ptr, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_mul(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildMul(self.ptr, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_sdiv(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef,
                      name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildSDiv(self.ptr, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_icmp(&mut self, op: LLVMIntPredicate, lhs: LLVMValueRef, rhs: LLVMValueRef,
                      name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildICmp(self.ptr, op, lhs, rhs, c_name.as_ptr())
        }
    }

    pub fn build_global_string(&self, s: &str, name: &str) -> LLVMValueRef {
        let c_s = CString::new(s).unwrap();
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildGlobalString(self.ptr, c_s.as_ptr(), c_name.as_ptr())
        }
    }

    pub fn build_in_bounds_gep(&self, ptr: LLVMValueRef, mut indices: Vec<LLVMValueRef>,
                               name: &str) -> LLVMValueRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMBuildInBoundsGEP(self.ptr, ptr, indices.as_mut_ptr(),
                                       indices.len() as u32, c_name.as_ptr())
        }
    }


}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMDisposeBuilder(self.ptr);
        }
    }
}

