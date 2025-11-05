 sqlx::query!(
     r#"
         select exists (
       		select "id"
       		from "event"
       		where
      			"reason" = 'fall_detection'
                 and "deviceID" = $1
      			and (
                     ("hasApprovalByHumanInTheLoop" or (not "needsApprovalByHumanInTheLoop")) and "createdAt" > now() - interval '5 minutes'
                     or
                     ("needsApprovalByHumanInTheLoop" and "createdAt" > now() - interval '120 seconds')
                 )
         ) as "exists!"
     "#,
     self.id
 )
 .fetch_one(pool)
 .await
 .map_err(|e| warn!("fails to get recent fall alarm event, error: {e:?}"))
 .map(|v| v.exists)
 .unwrap_or(false)
