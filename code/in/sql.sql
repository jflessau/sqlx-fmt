select exists(
    select "id"
    from "event"
    where
        "reason" = 'fall_detection'
        and "deviceID" = $1
        and (
            (
                "hasApprovalByHumanInTheLoop"
                or (not "needsApprovalByHumanInTheLoop")
            )
            and "createdAt" > now() - interval '5 minutes'
            or
            (
                "needsApprovalByHumanInTheLoop"
                and "createdAt" > now() - interval '120 seconds'
            )
        )
) as "exists!"
